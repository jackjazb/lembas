import { useEffect } from 'react';
import { View, TextInput, ScrollView, KeyboardAvoidingView } from 'react-native';
import { Button, Card, IconButton, List, Text, useTheme, HelperText, TextInput as MaterialTextInput } from 'react-native-paper';
import { useAppDispatch, useAppSelector } from 'app/hooks';
import { EditMode, editRecipeActions } from 'features/recipe/editRecipeSlice';
import { StackActions, useNavigation } from '@react-navigation/native';
import { showToast } from 'features/common/utils';
import { NumberPicker } from 'features/common/components/NumberPicker';
import { IngredientQuantityEditable, createRecipe, fromRecipeEditable, updateRecipe } from 'lembas-api';
import { recipeActions } from 'features/recipe/recipeSlice';

interface StepInputProps {
	index: number;
	value: string;
	onChangeText: (index: number, text: string) => void;
	onSubmit: (index: number) => void;
	onDelete: (index: number) => void;
}

/**
 * An input for adding plaintext steps to recipes.
 */
function StepInput(props: StepInputProps) {
	const { index, value, onChangeText, onDelete, onSubmit } = props;
	const theme = useTheme();

	return (
		<View style={{ flexDirection: 'row', flex: 1, alignItems: 'center' }}>
			<Text variant="bodyLarge">{`${index + 1}. `}</Text>
			<TextInput
				style={{
					fontSize: theme.fonts.bodyLarge.fontSize,
					flexShrink: 1,
					flex: 1,
				}}
				multiline
				blurOnSubmit={true}
				value={value}
				onChangeText={(text) => onChangeText(index, text)}
				onSubmitEditing={() => onSubmit(index)}
				placeholder="Step" />
			<IconButton style={{ marginLeft: 'auto' }} icon="delete-outline" onPress={() => onDelete(index)} />
		</View>
	);
}

interface IngredientInputProps {
	ingredient: IngredientQuantityEditable,
	quantityError: boolean;
	onChangeQuantity: (text: string) => void;
	onDelete: () => void;
}

/**
 * An input for modifying the quantity of an ingredient.
 */
function IngredientInput(props: IngredientInputProps): JSX.Element {
	const { ingredient, quantityError, onChangeQuantity, onDelete } = props;

	let helperText: JSX.Element;
	if (quantityError) {
		helperText = (
			<HelperText type='error' visible={true}>
				Number required.
			</HelperText>
		);
	}
	else {
		helperText = (
			<HelperText type='info' visible={true}>
				{ingredient.ingredient.unit}
			</HelperText>
		);
	}
	return (
		<View style={{ flexDirection: 'row', alignItems: 'center', gap: 10 }}>
			<List.Icon icon="package-variant-closed" />
			<Text variant="bodyLarge" style={{ flex: 1 }}>{ingredient.ingredient.name}</Text>

			<View>
				<HelperText type="error" visible={false/**quantityError*/}>
					Spacing fix.
				</HelperText>
				<MaterialTextInput
					keyboardType='numeric'
					style={{
						height: 30
					}}
					value={ingredient.quantity.toString()}
					onChangeText={onChangeQuantity}
				/>
				{helperText}
			</View >
			<IconButton
				icon="delete-outline"
				onPress={onDelete}
			/>
		</View >
	);
}

/**
 * Renders a component for editing or creating recipes.
 */
export function EditRecipe() {
	const dispatch = useAppDispatch();
	const navigation = useNavigation();

	const { editMode, recipe, errors } = useAppSelector(state => state.editRecipe);
	const theme = useTheme();

	// Switch screens to select ingredients
	const onAddIngredient = () => {
		navigation.dispatch(StackActions.push('Select Ingredient'));
	};

	const setQuantity = (quantity: string, ingredient: IngredientQuantityEditable) => {
		dispatch(editRecipeActions.setQuantityById({
			id: ingredient.ingredient.id,
			quantity
		}));
	};

	const setStep = (index: number, text: string) => {
		dispatch(editRecipeActions.setStep(index, text));
	};

	const addStep = (index: number) => {
		// Only add a step if called from the final text box
		if (index === recipe.steps.length - 1) {
			dispatch(editRecipeActions.addStep());
		}
	};

	const deleteStep = (index: number) => {
		dispatch(editRecipeActions.deleteStep(index));
	};

	const hasErrors = () => {
		return errors.length > 0
			|| recipe.name.length === 0
			|| recipe.ingredients.length === 0
			|| recipe.steps.length === 0;
	};

	const onSave = async () => {
		// These should both be fine, as the button will be disabled if any errors are present.
		if (editMode === EditMode.Create) {
			const typedRecipe = fromRecipeEditable(recipe);
			try {
				await createRecipe(typedRecipe);
				dispatch(recipeActions.syncRecipes());
				navigation.dispatch(StackActions.popToTop());
				showToast('Recipe saved');
			}
			catch (e) {
				showToast('Network error');
			}

		}
		if (editMode === EditMode.Edit) {
			const typedRecipe = fromRecipeEditable(recipe);
			const status = await updateRecipe(typedRecipe);
			if (status === 204) {
				dispatch(recipeActions.syncRecipes());
				navigation.dispatch(StackActions.popToTop());
				showToast('Recipe updated');
			}

		}
	};

	// Add a 'Save' button to the header
	useEffect(() => {
		navigation.setOptions({
			headerRight: () => (
				<IconButton
					icon="check"
					disabled={hasErrors()}
					onPress={onSave} />
			),
		});


	}, [navigation, recipe, editMode]);

	return (
		<ScrollView >
			<KeyboardAvoidingView>
				<View style={{ margin: 10, gap: 10 }}>
					<TextInput
						style={{
							fontSize: theme.fonts.displaySmall.fontSize,
							marginVertical: 10,

						}}
						value={recipe.name}
						onChangeText={(text) => dispatch(editRecipeActions.setTitle(text))}
						placeholder="Title" />

					<View style={{ flexDirection: 'row', alignItems: 'center' }}>
						<Text variant="titleLarge">Portions</Text>
						<NumberPicker
							value={recipe.portions} onChange={function (value: number): void {
								dispatch(editRecipeActions.setPortions(value));
							}} />
					</View>
					<Card mode="contained">
						<Card.Title titleVariant="titleLarge" title="Ingredients" />
						<Card.Content>
							{recipe.ingredients.map((ingredient, index) =>
								<IngredientInput
									key={index}
									ingredient={ingredient}
									onChangeQuantity={input => setQuantity(input, ingredient)}
									quantityError={errors.includes(ingredient.ingredient.id)}
									onDelete={() => dispatch(editRecipeActions.deleteIngredientById(ingredient.ingredient.id))}
								/>
								// <List.Item
								// 	key={index}
								// 	title={ingredient.ingredient.name}
								// 	left={() => <List.Icon icon="package-variant-closed" />}
								// 	right={() =>
								// 		<View style={{
								// 			flexDirection: 'row',
								// 			marginLeft: 'auto',
								// 			width: '50%',
								// 			gap: 5,
								// 		}}>
								// 			<View >
								// 				<View style={{ flexDirection: 'row' }}>
								// 					<TextInput
								// 						keyboardType='numeric'
								// 						style={{
								// 							backgroundColor: theme.colors.tertiaryContainer,
								// 							fontSize: theme.fonts.bodyLarge.fontSize,
								// 							width: '100%',

								// 						}}
								// 						value={ingredient.quantity.toString()}
								// 						onChangeText={input => setQuantity(input, ingredient)}
								// 					/>
								// 					<Text variant="bodyLarge">{ingredient.ingredient.unit}</Text>
								// 				</View>
								// 				<HelperText type="error" visible={errors.includes(ingredient.ingredient.id)}>
								// 					Number required.
								// 				</HelperText>
								// 			</View>
								// 			<IconButton
								// 				icon="delete-outline"

								// 				onPress={() => dispatch(editRecipeActions.deleteIngredientById(ingredient.ingredient.id))} />
								// 		</View>
								// 	}
								// />
							)}

							<Button icon="plus" onPress={onAddIngredient}>Add</Button>

						</Card.Content>
					</Card>

					<Card mode="contained">
						<Card.Title titleVariant="titleLarge" title="Steps" />
						<Card.Content>
							<View style={{ gap: 10 }}>
								{recipe.steps.map((step, ai) =>
									<StepInput
										key={ai}
										index={ai}
										value={recipe.steps[ai]}
										onChangeText={setStep}
										onSubmit={() => { }}
										onDelete={deleteStep}
									/>
								)}
								<Button icon="plus" onPress={() => addStep(recipe.steps.length - 1)}>Add</Button>
								<View style={{ flexDirection: 'row', alignItems: 'center' }}>

								</View>
							</View>
						</Card.Content>

					</Card>
				</View>
			</KeyboardAvoidingView>
		</ScrollView >
	);
}
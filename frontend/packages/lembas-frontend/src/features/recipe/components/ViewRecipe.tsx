import { StackActions } from '@react-navigation/native';
import { deleteRecipe } from 'lembas-api';
import { useAppDispatch, useAppSelector } from 'app/hooks';
import { DeleteConfirmModal } from 'features/common/components/DeleteConfirmModal';
import { scaleToPortions, showToast } from 'features/common/utils';
import { IngredientQuantityList } from 'features/ingredient/components/IngredientQuantityList';
import { recipeActions } from 'features/recipe/recipeSlice';
import { useEffect, useState } from 'react';
import { ScrollView, View } from 'react-native';
import { Card, IconButton, Portal, Text, useTheme } from 'react-native-paper';
import { NumberPicker } from 'features/common/components/NumberPicker';

/**
 * Renders a read-only version of 
 * @returns 
 */
export function ViewRecipe({ navigation }) {
	const theme = useTheme();
	const dispatch = useAppDispatch();
	const recipe = useAppSelector(state => state.recipes.selected);

	// If the delete modal is open.
	const [deleteOpen, setDeleteOpen] = useState(false);

	const [subMinimum, setSubMinimum] = useState(false);

	const scaleRecipe = (portions) => {
		// Scale the recipe.
		// If any ingredients are below their minimum quantities, flag it.
		const [scaledRecipe, subMinimum] = scaleToPortions(recipe, portions);
		setSubMinimum(subMinimum);
		dispatch(recipeActions.setSelectedRecipe(scaledRecipe));
	};

	const confirmDelete = async () => {
		// Delete the current recipe and return to the list screen.
		try {
			await deleteRecipe(recipe.id);
			dispatch(recipeActions.syncRecipes());
			navigation.dispatch(StackActions.popToTop());
			showToast('Recipe deleted');
		}
		catch (e) {
			showToast('Network error');
		}

	};

	const cancelDelete = async () => {
		setDeleteOpen(false);
	};

	// Modify the navigation object in order to add a 'Delete' button to the header
	useEffect(() => {
		navigation.setOptions({
			headerRight: () => (
				<IconButton icon="delete-outline" onPress={() => {
					setDeleteOpen(true);

				}} />
			),
		});
	}, [navigation, recipe]);

	const steps = recipe.steps.map((step, i) => (
		<View key={i} style={{ flexDirection: 'row', gap: 10, padding: 10, paddingLeft: 5 }}>
			<Text style={{ flexWrap: 'wrap' }} variant='titleMedium'>{`${i + 1}.`}</Text>
			<Text style={{ flexWrap: 'wrap' }} variant='bodyLarge'>{step}</Text>
		</View >
	));
	return (
		<View>
			<ScrollView>
				<View style={{ margin: 10, gap: 10 }}>
					<Portal>
						<DeleteConfirmModal
							title="Delete this recipe?"
							open={deleteOpen}
							onConfirm={confirmDelete}
							onDismiss={cancelDelete} />
					</Portal>
					<Card mode='contained'>
						<Card.Title
							title="Portions"
							titleVariant="titleLarge"
							right={() =>
								<NumberPicker
									lbound={1}
									value={recipe.portions}
									onChange={scaleRecipe} />
							}
						/>
						<Card.Content>
							<Text variant='bodySmall' style={{ color: theme.colors.error }}>
								{subMinimum && 'Some ingredients below minimum usable quantity'}
							</Text>
						</Card.Content>
					</Card>
					<Card mode='contained'>
						<Card.Title
							title="Ingredients"
							titleVariant="titleLarge"
						/>
						<Card.Content>
							<IngredientQuantityList ingredients={recipe.ingredients} />
						</Card.Content>
					</Card>

					<Card mode='contained'>
						<Card.Title title="Steps" titleVariant="titleLarge" />
						<Card.Content>
							{steps}
						</Card.Content>
					</Card>
				</View>
			</ScrollView >

		</View>
	);
}

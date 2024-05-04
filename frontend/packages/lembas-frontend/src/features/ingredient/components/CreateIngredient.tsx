import { StackActions, useNavigation } from '@react-navigation/native';
import { createUserIngredient, fromIngredientEditable } from 'lembas-api';
import React from 'react';
import { View, TextInput } from 'react-native';
import { Text, useTheme, Button, TextInput as MaterialTextInput, Card } from 'react-native-paper';
import { useAppDispatch, useAppSelector } from 'app/hooks';
import { showToast } from 'features/common/utils';
import { ingredientActions, ingredientSelectors } from 'features/ingredient/ingredientSlice';


export function CreateIngredient(): JSX.Element {
	const dispatch = useAppDispatch();
	const theme = useTheme();
	const navigation = useNavigation();

	const { selected } = useAppSelector(state => state.ingredients);
	const selectedHasErrors = useAppSelector(ingredientSelectors.selectedIngredientHasErrors);


	const onConfirm = async () => {
		const ingredient = fromIngredientEditable(selected);

		// This should never be hit, as the button will be disabled if there is an error.
		if (!ingredient) {
			return;
		}
		try {
			await createUserIngredient(ingredient);

			navigation.dispatch(StackActions.popToTop());
			dispatch(ingredientActions.resetSelectedIngredient());
			dispatch(ingredientActions.syncUserIngredients());
			showToast('Custom ingredient created');
		}
		catch (e) {
			showToast('Network error');
		}

	};

	return (
		<Card mode="contained" style={{ margin: 10 }}>
			<Card.Content>
				<View style={{ flexDirection: 'row', alignItems: 'center' }}>
					<TextInput
						style={{ fontSize: theme.fonts.titleLarge.fontSize }}
						onChangeText={(t) => dispatch(ingredientActions.setSelectedIngredientName(t))}
						placeholder='Name'
						value={selected?.name}
					/>
				</View>
				<View style={{ flexDirection: 'row', alignItems: 'center', justifyContent: 'space-between' }}>
					<Text variant="bodyLarge" >
						Unit
					</Text>
					<MaterialTextInput
						onChangeText={(t) => dispatch(ingredientActions.setSelectedIngredientUnit(t))}
						value={selected.unit}
					/>
				</View>
				<View style={{ flexDirection: 'row', alignItems: 'center', justifyContent: 'space-between' }}>
					<Text variant="bodyLarge" >
						Life
					</Text>
					<MaterialTextInput
						keyboardType='numeric'
						onChangeText={(t) => dispatch(ingredientActions.setSelectedIngredientLife(t))}
						value={selected.life}
					/>
				</View>
				<View style={{ flexDirection: 'row', alignItems: 'center', justifyContent: 'space-between' }}>
					<Text variant="bodyLarge" >
						Minimum purchase
					</Text>
					<MaterialTextInput
						keyboardType='numeric'
						onChangeText={(t) => dispatch(ingredientActions.setSelectedIngredientMinPurchase(t))}
						value={selected.purchase_quantity}
					/>
				</View>
				<View style={{ flexDirection: 'row', alignItems: 'center', justifyContent: 'space-between' }}>
					<Text variant="bodyLarge" >
						Minimum usable
					</Text>
					<MaterialTextInput
						keyboardType='numeric'
						onChangeText={(t) => dispatch(ingredientActions.setSelectedIngredientMinQuantity(t))}
						value={selected.minimum_quantity}
					/>
				</View>

			</Card.Content>
			<Card.Actions style={{ marginTop: 10 }}>
				<Button mode="contained" disabled={selectedHasErrors} onPress={onConfirm}>
					Confirm
				</Button>
			</Card.Actions>
		</Card>
	);
}
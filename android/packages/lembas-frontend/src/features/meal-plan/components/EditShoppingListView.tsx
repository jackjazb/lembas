import { ShoppingListEditable, ShoppingListItem } from 'lembas-api';
import { Loading } from 'features/common/components/Loading';
import { useEffect } from 'react';
import { ScrollView, View } from 'react-native';
import { Divider, Text } from 'react-native-paper';
import { useAppDispatch, useAppSelector } from 'app/hooks';
import { mealPlanActions, mealPlanSelectors } from 'features/meal-plan/mealPlanSlice';
import { RequestStatus } from 'app/redux-utils';
import { ListIngredient } from 'features/meal-plan/components/TickableListIngredient';

// TODO - make each ingredient tickable or untickable regardless of whether you have it
// Check for instead of already have
export function EditShoppingListView() {
	const dispatch = useAppDispatch();
	const editableList = useAppSelector(mealPlanSelectors.selectShoppingListEditable);
	const listStatus = useAppSelector(mealPlanSelectors.selectListStatus);

	useEffect(() => {
		if (listStatus === RequestStatus.Idle) {
			dispatch(mealPlanActions.syncList());
		}
	}, [listStatus, dispatch]);


	const renderListItem = (ingredient: ShoppingListItem, key: keyof ShoppingListEditable, i: number) => {
		const divider = i === 0 ? undefined : <Divider />;
		return (
			<View style={{ marginVertical: 5, gap: 5 }} key={i} >
				{divider}
				<View style={{ width: '95%' }}>
					<ListIngredient
						tickableIngredient={ingredient}
						setTicked={() =>
							dispatch(mealPlanActions.toggleListIngredient({ key, i }))}
					/>
				</View>
			</View>
		);
	};

	function renderListSection(title: string, key: keyof ShoppingListEditable, list: ShoppingListEditable) {
		if (list[key].length === 0) {
			return undefined;
		}
		return (
			<>
				<Text variant='headlineSmall' style={{ fontWeight: 'bold' }}>{title}</Text>
				{
					list[key].map((ing, i) => renderListItem(ing, key, i))
				}
			</>
		);
	}
	if (listStatus === RequestStatus.Loading) {
		return <Loading />;
	}

	return (
		<ScrollView style={{ padding: 10, flex: 1 }}>
			<View style={{ gap: 10, marginBottom: 30 }}>
				{renderListSection('Need to Buy', 'ingredients', editableList)}
				{renderListSection('Restock Due', 'scheduledIngredients', editableList)}
				{renderListSection('Check For', 'checkFor', editableList)}
			</View>

		</ScrollView >
	);
}

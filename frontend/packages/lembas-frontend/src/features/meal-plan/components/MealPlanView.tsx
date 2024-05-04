import { Recipe, createDay, getISODateString, deleteRecipeFromDay, Day } from 'lembas-api';
import { useState, useEffect, useCallback } from 'react';
import { View, ScrollView, RefreshControl } from 'react-native';
import { Portal, FAB, Text, IconButton, Card, List } from 'react-native-paper';
import { StackActions, useNavigation } from '@react-navigation/native';
import { RecipeSelectionModal } from 'features/meal-plan/components/RecipeSelectionModal';
import { formatDate, getWeekday } from 'features/common/utils';
import { DaySelector } from 'features/meal-plan/components/DaySelector';
import { useAppDispatch, useAppSelector } from 'app/hooks';
import { mealPlanActions, mealPlanSelectors } from 'features/meal-plan/mealPlanSlice';
import { RequestStatus } from 'app/redux-utils';
import { Loading } from 'features/common/components/Loading';
import { DeleteConfirmModal } from 'features/common/components/DeleteConfirmModal';

/**
 * Renders the main meal planning view.
 */
export function MealPlanView() {
	const navigation = useNavigation();
	const dispatch = useAppDispatch();


	const { recipes } = useAppSelector(state => state.recipes);

	// Work out the date range for the current week.
	const { selectedDay, rangeStart, days, dayStatus } = useAppSelector(state => state.mealPlan);
	const rangeEnd = useAppSelector(mealPlanSelectors.selectRangeEnd);

	// Whether the modal for the above input is open
	const [modalOpen, setModalOpen] = useState(false);

	const [refreshing] = useState(false);

	const onRefresh = useCallback(() => {
		dispatch(mealPlanActions.syncDays());
	}, []);

	// Called when the date range changes
	useEffect(() => {
		if (dayStatus === RequestStatus.Idle) {
			dispatch(mealPlanActions.syncDays());
		}
	}, [dayStatus, dispatch]);

	async function adjustRange(days: number) {
		dispatch(mealPlanActions.adjustRange(days));
		dispatch(mealPlanActions.syncDays());
	}

	const refreshControl = <RefreshControl refreshing={refreshing} onRefresh={onRefresh} />;


	let content: JSX.Element;
	if (dayStatus === RequestStatus.Loading) {
		content = <Loading />;
	}
	else if (days.length === 0) {
		content = <Text variant='titleMedium'>No meals planned</Text>;
	}
	else {
		content = (
			<View style={{
				gap: 10
			}}>
				{days.map((day, i) =>
					<DayCard
						key={i}
						day={day}
						onRecipeDelete={async (recipe, date) => {
							await deleteRecipeFromDay(recipe, date);
							dispatch(mealPlanActions.syncDays());
						}}
					/>
				)}
			</View>
		);
	}

	return (
		<>
			{selectedDay &&
				<Portal>
					<RecipeSelectionModal
						open={modalOpen}
						day={new Date(selectedDay)}
						recipes={recipes}
						onDismiss={() => setModalOpen(false)}
						onConfirm={(recipe) => {
							if (recipe) {
								createDay(recipe, selectedDay);
								setModalOpen(false);
								dispatch(mealPlanActions.syncDays());
							}

						}}
					/>
				</Portal>
			}

			<ScrollView refreshControl={refreshControl}>
				<View style={{ margin: 10, gap: 10, flex: 1 }}>

					<Text variant='headlineLarge'>
						{'Week of '}
						<Text style={{ fontWeight: 'bold' }}>
							{`${formatDate(new Date(rangeStart))}`}
						</Text>
					</Text>
					<DaySelector
						firstDay={new Date(rangeStart)}
						planDays={days}
						onPressDay={(day) => {
							setModalOpen(true);
							dispatch(mealPlanActions.setSelectedDay(getISODateString(day)));
						}}
						onPressLeftArrow={() => adjustRange(-7)}
						onPressRightArrow={() => adjustRange(7)}
						resetEnabled={!useAppSelector(mealPlanSelectors.isCurrentWeek)}
						onPressReset={() => {
							dispatch(mealPlanActions.resetRange());
							dispatch(mealPlanActions.syncDays());
						}}
					/>
					<View style={{ flexGrow: 1 }}>
						{content}
					</View>
				</View>

			</ScrollView>
			<FAB
				icon='clipboard-list-outline'
				style={{
					position: 'absolute',
					margin: 16,
					right: 0,
					bottom: 0,
				}}
				onPress={() => {
					dispatch(mealPlanActions.syncList());
					navigation.dispatch(StackActions.push('Shopping List', {
						from: rangeStart,
						to: rangeEnd
					}));
				}
				}
			/>
		</>
	);
}

interface DayCardProps {
	day: Day;
	onRecipeDelete: (recipe, date) => void;
}
function DayCard(props: DayCardProps) {
	const { day, onRecipeDelete } = props;
	const [deleteOpen, setDeleteOpen] = useState(false);
	const [recipe, setRecipe] = useState<Recipe | undefined>(undefined);

	const date = new Date(day.date);
	return (
		<>
			<Portal>
				<DeleteConfirmModal
					title={'Delete planned recipe?'}
					open={deleteOpen}
					onConfirm={() =>
						onRecipeDelete(recipe, date)
					}
					onDismiss={() => setDeleteOpen(false)} />
			</Portal>
			<Card mode='contained'>
				<Card.Title titleVariant="titleLarge" title={getWeekday(date)} />
				<Card.Content>
					{day.recipes.map((r: Recipe, i: number) => (

						<List.Item
							key={i}
							title={r.name}
							left={() =>
								<List.Icon icon="notebook-outline" />
							}
							right={() =>
								<IconButton icon='delete-outline' style={{ marginLeft: 'auto' }} onPress={() => {
									setRecipe(r);
									setDeleteOpen(true);
								}} />

							} />

					))}
				</Card.Content>

			</Card >
		</>
	);
}
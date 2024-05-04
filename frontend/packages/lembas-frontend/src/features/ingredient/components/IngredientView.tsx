import { deleteScheduledIngredient, deleteUserIngredient } from 'lembas-api';
import { useAppDispatch, useAppSelector } from 'app/hooks';
import { RequestStatus } from 'app/redux-utils';
import { DeleteConfirmModal } from 'features/common/components/DeleteConfirmModal';
import { Loading } from 'features/common/components/Loading';
import { formatDate, getNextIntervalDate, showToast } from 'features/common/utils';
import { scheduledIngredientActions } from 'features/ingredient/scheduledIngredientSlice';
import React, { useState, useEffect } from 'react';
import { View, RefreshControl, ScrollView } from 'react-native';
import { Portal, Card, Modal, Text, Icon, FAB, List, IconButton } from 'react-native-paper';
import { StackActions, useNavigation } from '@react-navigation/native';
import { IngredientList } from 'features/ingredient/components/IngredientList';
import { ingredientActions } from 'features/ingredient/ingredientSlice';


export function IngredientView() {
	const dispatch = useAppDispatch();
	const navigation = useNavigation();

	const [deleteReminderOpen, setDeleteReminderOpen] = useState(false);
	const [deleteIngredientOpen, setDeleteIngredientOpen] = useState(false);
	const [editOpen, setEditOpen] = useState(false);

	const [refreshing] = useState(false);

	const { scheduledIngredients, selected: selectedReminder, status: scheduleStatus } = useAppSelector(state => state.scheduledIngredients);
	const { userIngredients, selected: selectedIngredient } = useAppSelector(state => state.ingredients);

	const { userIngredientStatus: ingredientStatus } = useAppSelector(state => state.ingredients);

	useEffect(() => {
		if (scheduleStatus === RequestStatus.Idle) {
			dispatch(scheduledIngredientActions.syncScheduledIngredients());
		}
		if (ingredientStatus === RequestStatus.Idle) {
			dispatch(ingredientActions.syncUserIngredients());
		}
	}, [scheduleStatus, ingredientStatus, dispatch]);

	const onRefresh = async () => {
		dispatch(scheduledIngredientActions.syncScheduledIngredients());
		dispatch(ingredientActions.syncUserIngredients());
	};

	const confirmDeleteReminder = async () => {
		try {
			await deleteScheduledIngredient(selectedReminder);
			dispatch(scheduledIngredientActions.syncScheduledIngredients());
			setDeleteReminderOpen(false);
			showToast('Reminder deleted');
		}
		catch (e) {
			showToast('Network error');
		}
	};

	const confirmDeleteIngredient = async () => {
		try {
			await deleteUserIngredient(selectedIngredient.id);
			setDeleteIngredientOpen(false);
			dispatch(ingredientActions.syncUserIngredients());
			dispatch(ingredientActions.resetSelectedIngredient());
			showToast('Ingredient deleted');
		}
		catch (e) {
			showToast('Network error');
		}
	};

	const cancelDeleteReminder = () => {
		setDeleteReminderOpen(false);
	};

	const cancelDeleteIngredient = () => {
		setDeleteIngredientOpen(false);
	};

	const refreshControl = <RefreshControl refreshing={refreshing} onRefresh={onRefresh} />;

	let scheduleContent: JSX.Element;

	if (scheduleStatus === RequestStatus.Loading) {
		scheduleContent = <Loading />;
	}
	else if (scheduleStatus === RequestStatus.Succeeded && scheduledIngredients.length === 0) {
		scheduleContent = (
			<View style={{ alignItems: 'center', flex: 1, justifyContent: 'center' }}>
				<Text variant='titleMedium'>No reminders set.</Text>
				<View style={{ flexDirection: 'row', alignItems: 'center' }}>
					<Text variant='bodyMedium'>Tap </Text>
					<Icon size={24} source="magnify" />
					<Text variant='bodyLarge'> to create one.</Text>
				</View >
			</View>
		);
	}
	else {
		scheduleContent = (
			<>
				{scheduledIngredients.map((ingredient, i) =>
					<List.Item key={i}
						title={ingredient.ingredient.name}
						description={() => (<Text variant="labelMedium">
							<Text>
								{`Every ${ingredient.interval} days\n`}
							</Text>
							<Text>
								{`Next reminder on ${formatDate(getNextIntervalDate(new Date(ingredient.start_date), ingredient.interval))}`}
							</Text>
						</Text>)}
						left={props =>
							< List.Icon {...props} icon="package-variant-closed" />
						}
						right={() =>
							<IconButton onPress={() => {
								dispatch(scheduledIngredientActions.setSelectedScheduledIngredient(ingredient));
								setDeleteReminderOpen(true);
							}}
								icon="delete-outline" />
						}
					/>)
				}
			</>);
	}

	let ingredientContent: JSX.Element;

	if (ingredientStatus === RequestStatus.Loading) {
		ingredientContent = <Loading />;
	}
	else if (ingredientStatus === RequestStatus.Succeeded && userIngredients.length === 0) {
		ingredientContent = (
			<View style={{ alignItems: 'center', flex: 1, justifyContent: 'center' }}>
				<Text variant='titleMedium'>No custom ingredients.</Text>
				<View style={{ flexDirection: 'row', alignItems: 'center' }}>
					<Text variant='bodyMedium'>Tap </Text>
					<Icon size={24} source="pencil" />
					<Text variant='bodyLarge'> to create one.</Text>
				</View >
			</View>
		);
	}
	else {
		ingredientContent = (
			<IngredientList
				scrolling={false}
				ingredients={userIngredients.filter(ing => ing.user_id)}
				refreshControl={undefined}
				actions={[
					{
						icon: 'delete-outline',
						callback: (ingredient) => {
							dispatch(ingredientActions.setSelectedIngredient(ingredient));
							setDeleteIngredientOpen(true);
						}
					}
				]}
			/>
		);
	}
	return (
		<>
			<ScrollView contentContainerStyle={{ margin: 15, gap: 20 }} refreshControl={refreshControl}>
				<Portal>
					<DeleteConfirmModal
						title="Delete this reminder?"
						open={deleteReminderOpen}
						onConfirm={confirmDeleteReminder}
						onDismiss={cancelDeleteReminder}
					/>
					<DeleteConfirmModal
						title="Delete this ingredient?"
						open={deleteIngredientOpen}
						onConfirm={confirmDeleteIngredient}
						onDismiss={cancelDeleteIngredient}
					/>

					<Modal visible={editOpen} onDismiss={() => setEditOpen(false)} contentContainerStyle={{ padding: 20 }}>
						<Card mode='contained' >

							<Card.Content style={{ gap: 10 }}>
								<Text variant='titleLarge'>
									{'Edit reminder'}
								</Text>
							</Card.Content>
						</Card>
					</Modal>
				</Portal>

				<Text variant="titleLarge" style={{ fontWeight: 'bold' }}>Reminders</Text>
				{scheduleContent}
				<Text variant="titleLarge" style={{ fontWeight: 'bold' }}>Custom Ingredients</Text>
				{ingredientContent}
				<View style={{ height: 60 }} />
			</ScrollView >
			<FAB
				icon='pencil'
				style={[{
					position: 'absolute',
					margin: 16,
					right: 0,
					bottom: 0,
				}]}
				onPress={() => {
					navigation.dispatch(StackActions.push('Create Ingredient'));

				}} label={''} />
		</>
	);
}
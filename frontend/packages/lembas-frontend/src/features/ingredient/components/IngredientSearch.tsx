import { createScheduledIngredient, getISODateString, fromIngredientEditable } from 'lembas-api';
import { Card, Portal, Text, Button, Modal, useTheme } from 'react-native-paper';
import { useState, useEffect, useCallback } from 'react';
import { RefreshControl, TextInput, View } from 'react-native';
import { DateTimePickerAndroid } from '@react-native-community/datetimepicker';
import { StackActions } from '@react-navigation/native';
import React from 'react';
import { useAppDispatch, useAppSelector } from 'app/hooks';
import { RequestStatus } from 'app/redux-utils';
import { Loading } from 'features/common/components/Loading';
import { NumberPicker } from 'features/common/components/NumberPicker';
import { showToast, formatDate } from 'features/common/utils';
import { ingredientSearchActions } from 'features/ingredient/ingredientSearchSlice';
import { ingredientActions } from 'features/ingredient/ingredientSlice';
import { scheduledIngredientActions } from 'features/ingredient/scheduledIngredientSlice';
import { IngredientList } from 'features/ingredient/components/IngredientList';

export function IngredientSearchImpl({ navigation }) {
	const dispatch = useAppDispatch();
	const theme = useTheme();

	const [text, setText] = useState('');
	const isSearching = text !== '';

	const { selected } = useAppSelector(state => state.ingredients);

	const { results, status } = useAppSelector(state => state.ingredientSearch);

	// Scheduling modal params.
	const [scheduleOpen, setScheduleOpen] = useState(false);
	const [date, setDate] = useState(new Date(Date.now()));
	const [interval, setInterval] = useState(7);

	// These three functions are to do with the date picker.
	const onDateChange = useCallback((event, selectedDate) => {
		const currentDate = selectedDate;
		setDate(currentDate);
	}, []);

	const showMode = (currentMode) => {
		DateTimePickerAndroid.open({
			value: date,
			onChange: onDateChange,
			mode: currentMode,
		});
	};

	const showDatepicker = () => {
		showMode('date');
	};

	// Set header bar contents
	useEffect(() => {
		navigation.setOptions({
			headerTitle: () => (
				<TextInput
					style={{
						fontSize: theme.fonts.titleMedium.fontSize,
						flex: 1
					}}
					placeholder="Search"
					onChangeText={setText}
					value={text}
					autoFocus
				/>
			),
		});
	});

	useEffect(() => {
		if (isSearching) {
			dispatch(ingredientSearchActions.syncResults(text));
		}
	}, [text, dispatch]);

	const openScheduleModal = useCallback((ingredient) => {
		dispatch(ingredientActions.setSelectedIngredient(ingredient));
		setScheduleOpen(true);
	}, []);

	const onRefresh = () => {
		dispatch(ingredientActions.syncIngredients());
	};

	const onCreateReminder = async () => {
		const ingredient = fromIngredientEditable(selected);
		try {
			await createScheduledIngredient(ingredient, getISODateString(date), interval);
			setScheduleOpen(false);
			navigation.dispatch(StackActions.popToTop());
			dispatch(scheduledIngredientActions.syncScheduledIngredients());
			dispatch(ingredientActions.resetSelectedIngredient());

			showToast('Reminder created');
		} catch (e) {
			showToast('Network error');
		}

	};

	let content: React.ReactNode;
	if (!isSearching) {
		content = <View style={{
			flex: 1,
			alignSelf: 'center',
		}}>
			<Text
				variant='bodyLarge'
				style={{
					margin: 12,
				}}
			>
				Type to search...
			</Text>
		</View>;
	}
	else if (status === RequestStatus.Loading) {
		content = <Loading />;
	}
	// A query has been entered and no results were returned
	else if (isSearching && results.length === 0) {
		content = <Text
			variant='titleMedium'
			style={{
				margin: 12,
				justifyContent: 'center',
				alignSelf: 'center'
			}}
		>
			No matching ingredients
		</Text>;

		// Show a list with either all ingredients or the search results
	} else {
		content =
			<IngredientList
				refreshControl={<RefreshControl refreshing={status as RequestStatus === RequestStatus.Loading} onRefresh={onRefresh} />}
				ingredients={results}
				actions={[
					{
						icon: 'clock-outline',
						callback: openScheduleModal
					}
				]} />;
	}


	let modals;
	if (selected) {
		modals = <Portal>
			<Modal
				visible={scheduleOpen}
				onDismiss={() => setScheduleOpen(false)}
				contentContainerStyle={{ padding: 20 }}>
				<Card mode='contained' >
					<Card.Title titleVariant="titleLarge" title="Add reminder" />
					<Card.Content style={{ gap: 12, alignContent: 'space-between' }}>
						<View style={{ flexDirection: 'row', alignItems: 'center' }}>
							<Text variant="bodyLarge">
								<Text>For </Text>
								<Text style={{ fontWeight: 'bold' }}>{`${selected && selected.name}`}</Text>
							</Text>
						</View>

						<View style={{ flexDirection: 'row', alignItems: 'center' }}>
							<Text variant='bodyLarge' style={{ flex: 1 }}>
								<Text>Starting </Text>
								<Text style={{ fontWeight: 'bold' }}>
									{date.getDate() === new Date(Date.now()).getDate()
										? 'Today'
										: `${formatDate(date)}`}
								</Text>

							</Text>
							<Button onPress={showDatepicker}>Change</Button>
						</View>
						<View style={{ flexDirection: 'row', alignItems: 'center' }}>
							<Text variant='bodyLarge' style={{ flex: 1 }}>
								<Text>Every </Text>
								<Text style={{ fontWeight: 'bold' }}>{`${interval} Days`}</Text>

							</Text>
							<NumberPicker value={interval} onChange={setInterval} hideValue />
						</View>
					</Card.Content>

					<Card.Actions style={{ marginTop: 10 }}>
						<Button onPress={() => {
							setScheduleOpen(false);
							setDate(new Date(Date.now()));
							setInterval(7);
						}}>
							Cancel
						</Button>
						<Button onPress={onCreateReminder}>
							Confirm
						</Button>
					</Card.Actions>
				</Card>
			</Modal>
		</Portal>;
	}
	return (
		<>
			{modals}
			{content}

		</>
	);
}

export const IngredientSearch = React.memo(IngredientSearchImpl);
import { Recipe } from 'lembas-api';
import { View, ScrollView, } from 'react-native';
import { Card, Button, Modal, Text, List } from 'react-native-paper';
import { useState } from 'react';
import { formatDateWithDay } from 'features/common/utils';

export interface RecipeSelectionModalProps {
	/**
	 * Whether or not the modal should be open.
	 */
	open: boolean;
	/**
	 * The day being edited.
	 */
	day: Date;
	/**
	 * The recipes to choose from
	 */
	recipes: Recipe[];
	/**
	 * Called when the modal is closed.
	 */
	onDismiss: () => void;
	/**
	 * Called with the currently selected recipe when confirm is pressed.
	 */
	onConfirm: (recipe: Recipe) => void;
}

/**
 * Renders a recipe selection dialog for a given day.
 */
export function RecipeSelectionModal(props: RecipeSelectionModalProps) {
	const { open, day, recipes, onDismiss, onConfirm } = props;


	const [currentRecipe, setCurrentRecipe] = useState<Recipe | undefined>();

	// Called when the modal is dismissed.
	function dismiss() {
		if (currentRecipe) {
			setCurrentRecipe(undefined);
		}
		else {
			setCurrentRecipe(undefined);
			onDismiss();
		}

	}

	// Called when 'Confirm' is pressed.
	function confirm() {
		onConfirm(currentRecipe);
		setCurrentRecipe(undefined);
	}

	return (
		<Modal visible={open} onDismiss={() => onDismiss()} contentContainerStyle={{ padding: 20 }}>
			<Card mode='contained' >
				<Card.Content style={{ gap: 10 }}>
					<Text variant='headlineMedium'>{`${formatDateWithDay(day)}`}</Text>
					{!currentRecipe ?
						<View>
							<Text variant='titleLarge'>{'Select a Recipe'}</Text>

							<ScrollView style={{ maxHeight: 150 }}>
								{recipes.map((r, i) =>
									<List.Item
										key={i}
										title={r.name}
										onPress={() => setCurrentRecipe(r)}
										left={() =>
											<List.Icon icon="notebook-outline" />
										}
									/>
								)}
							</ScrollView>
						</View>
						:
						<Text variant='bodyLarge'>{'Add ' + currentRecipe.name + '?'}</Text>
					}
				</Card.Content>
				<Card.Actions style={{ marginTop: 10 }}>
					<Button onPress={() => dismiss()}>Cancel</Button>
					<Button onPress={() => confirm()} disabled={!currentRecipe}>Confirm</Button>
				</Card.Actions>
			</Card>
		</Modal >
	);
}
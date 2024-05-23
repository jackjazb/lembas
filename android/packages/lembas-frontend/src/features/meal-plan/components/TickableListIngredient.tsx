import { Ingredient, ShoppingListItem } from 'lembas-api';
import { ProgressIndicator } from 'features/common/components/ProgressIndicator';
import { getUnitString } from 'features/common/utils';
import { View } from 'react-native';
import { Checkbox, Text } from 'react-native-paper';

export interface TickableListIngredientProps {
	tickableIngredient: ShoppingListItem;
	setTicked: () => void;
}
export function ListIngredient(props: TickableListIngredientProps) {
	const { tickableIngredient, setTicked } = props;

	if (!tickableIngredient) {
		return undefined;
	}

	return (
		<View style={{ flexDirection: 'row', flex: 1 }}>
			<Checkbox
				status={tickableIngredient.ticked ? 'checked' : 'unchecked'}
				onPress={() => setTicked()}
			/>

			<View style={{ flex: 1 }}>
				{tickableIngredient.purchase_quantity > 0 ?
					<View style={{ gap: 10 }}>
						<Text variant='titleMedium'>{formatQuantity(tickableIngredient.ingredient, tickableIngredient.purchase_quantity)}</Text>
						<Text variant='labelLarge'>
							{
								`Need: ${tickableIngredient.used_quantity}${getUnitString(tickableIngredient.ingredient)}, ` +
								`Have: ${tickableIngredient.existing_surplus}${getUnitString(tickableIngredient.ingredient)}`
							}
						</Text>
						<ProgressIndicator value={tickableIngredient.existing_surplus} max={tickableIngredient.used_quantity} /></View>
					:
					<View style={{ gap: 10 }}>
						<Text variant='titleMedium'>{formatQuantity(tickableIngredient.ingredient, tickableIngredient.used_quantity)}</Text>
						<Text variant='labelLarge'>
							{
								`Have: ${tickableIngredient.existing_surplus}${getUnitString(tickableIngredient.ingredient)}`
							}
						</Text>
						<ProgressIndicator value={tickableIngredient.used_quantity} max={tickableIngredient.existing_surplus} />
					</View>
				}


			</View>
		</View>
	);
}

function formatQuantity(ingredient: Ingredient, quantity: number): string {
	const unit = getUnitString(ingredient);

	return `${ingredient.name}, ${quantity}${unit}`;
}
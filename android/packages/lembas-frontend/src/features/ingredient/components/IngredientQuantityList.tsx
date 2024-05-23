import { IngredientQuantity } from 'lembas-api';
import React from 'react';
import { ScrollView } from 'react-native';
import { List, Text } from 'react-native-paper';
import { formatIngredientName, formatIngredientQuantity } from 'features/common/utils';

export interface IngredientQuantityListProps {
	ingredients: IngredientQuantity[];
}

/**
 * Renders a list of ingredients with quantities.
 */
export function IngredientQuantityList(props: IngredientQuantityListProps) {
	const { ingredients } = props;

	return (
		<ScrollView>{ingredients.map((ingredient, i) =>
			<List.Item key={i}
				title={formatIngredientName(ingredient)}
				left={props => <List.Icon {...props} icon="package-variant-closed" />}
				right={() => <Text style={{ fontWeight: 'bold' }} variant='bodyLarge'>{formatIngredientQuantity(ingredient)}</Text>}
			/>)}
		</ScrollView>
	);
}
import { Ingredient } from 'lembas-api';
import React, { useCallback } from 'react';
import { FlatList, View } from 'react-native';
import { IconButton } from 'react-native-paper';
import { IngredientItem } from 'features/ingredient/components/IngredientItem';

export interface IngredientListProps {
	ingredients: Ingredient[];
	refreshControl: JSX.Element;
	actions: {
		icon: string;
		callback: (ingredient: Ingredient) => void;
	}[];
	scrolling?: boolean;
}

/**
 * Renders a list of ingredients in a virtualised list. 
 */
export function IngredientListImpl(props: IngredientListProps) {
	const { ingredients, actions, refreshControl, scrolling = true } = props;

	const keyExtractor = useCallback((item: Ingredient) => item.id.toString(), []);

	const renderActions = useCallback((item: Ingredient) =>
		<View style={{ flexDirection: 'row' }}>
			{actions.map(({ icon, callback }, i) =>
				<IconButton key={i} onPress={() => callback(item)} icon={icon} />
			)}
		</View>, []);

	const renderIngredient = useCallback(({ item }) =>
		<IngredientItem
			ingredient={item}
			right={() => renderActions(item)} />
		, []);

	if (!scrolling) {
		return (
			<View>
				{ingredients.map(ing => <View key={ing.id}>{renderIngredient({ item: ing })}</View>)}
			</View >
		);
	}

	return (
		<FlatList
			refreshControl={refreshControl}
			data={ingredients}
			renderItem={renderIngredient}
			keyExtractor={keyExtractor}
		/>
	);
}

export const IngredientList = React.memo(IngredientListImpl);
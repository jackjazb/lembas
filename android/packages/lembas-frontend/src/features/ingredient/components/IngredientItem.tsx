import { Ingredient } from 'lembas-api';
import { ReactNode } from 'react';
import { List } from 'react-native-paper';

export interface IngredientProps {
	ingredient: Ingredient;
	right?: () => ReactNode;
	onPress?: () => void;
}
export function IngredientItem(props: IngredientProps): JSX.Element {
	const { ingredient, right, onPress } = props;
	const icon = (props) => ingredient.user_id ? <List.Icon {...props} icon="account" /> : <List.Icon {...props} icon="package-variant-closed" />;

	return (
		<List.Item
			onPress={onPress}
			title={ingredient.name}
			right={right}
			left={icon}
		/>);
}
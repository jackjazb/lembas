import { Button, Card, List } from 'react-native-paper';
import { StackActions, useNavigation } from '@react-navigation/native';
import { Recipe } from 'lembas-api';
import { useAppDispatch } from 'app/hooks';
import { recipeActions } from 'features/recipe/recipeSlice';
import { EditMode, editRecipeActions } from 'features/recipe/editRecipeSlice';

// Grey 200x100 image.
const PLACEHOLDER =
	`data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMgAAABkCAYAAADDhn8LAAAAAXNSR0IArs4c6QAAAqVJREFU
	eF7t1bERwzAMBEGx/9bYk92A5eDSX+UIsOCNzr338/gIEPgpcATiZRB4FxCI10Hgj4BAPA8CAvEGCDQBf5DmZmpEQCAjh7Z
	mExBIczM1IiCQkUNbswkIpLmZGhEQyMihrdkEBNLcTI0ICGTk0NZsAgJpbqZGBAQycmhrNgGBNDdTIwICGTm0NZuAQJqbqR
	EBgYwc2ppNQCDNzdSIgEBGDm3NJiCQ5mZqREAgI4e2ZhMQSHMzNSIgkJFDW7MJCKS5mRoREMjIoa3ZBATS3EyNCAhk5NDWb
	AICaW6mRgQEMnJoazYBgTQ3UyMCAhk5tDWbgECam6kRAYGMHNqaTUAgzc3UiIBARg5tzSYgkOZmakRAICOHtmYTEEhzMzUi
	IJCRQ1uzCQikuZkaERDIyKGt2QQE0txMjQgIZOTQ1mwCAmlupkYEBDJyaGs2AYE0N1MjAgIZObQ1m4BAmpupEQGBjBzamk1
	AIM3N1IiAQEYObc0mIJDmZmpEQCAjh7ZmExBIczM1IiCQkUNbswkIpLmZGhEQyMihrdkEBNLcTI0ICGTk0NZsAgJpbqZGBA
	QycmhrNgGBNDdTIwICGTm0NZuAQJqbqREBgYwc2ppNQCDNzdSIgEBGDm3NJiCQ5mZqREAgI4e2ZhMQSHMzNSIgkJFDW7MJC
	KS5mRoREMjIoa3ZBATS3EyNCAhk5NDWbAICaW6mRgQEMnJoazYBgTQ3UyMCAhk5tDWbgECam6kRAYGMHNqaTUAgzc3UiIBA
	Rg5tzSYgkOZmakRAICOHtmYTEEhzMzUiIJCRQ1uzCQikuZkaERDIyKGt2QQE0txMjQgIZOTQ1mwCAmlupkYEBDJyaGs2AYE
	0N1MjAgIZObQ1m4BAmpupEQGBjBzamk1AIM3N1IjAFz+7UyCV+b6JAAAAAElFTkSuQmCC`;

export interface RecipeCardProps {
	recipe: Recipe;
	color: string;
}

/**
 * Renders a card containing a recipe title and button to interact with it.
 * @returns 
 */
export default function RecipeCard(props: RecipeCardProps): JSX.Element {
	const navigation = useNavigation();
	const dispatch = useAppDispatch();

	const { recipe, color } = props;

	return (
		<Card mode='contained'>
			<Card.Cover
				tintColor={color}
				source={{
					uri: PLACEHOLDER
				}}
				style={{ height: 140 }} />
			<Card.Title
				titleVariant="titleLarge"
				left={() => <List.Icon icon="notebook-outline" />}
				title={recipe.name} />

			<Card.Actions>
				<Button onPress={() => {
					dispatch(editRecipeActions.setEditMode(EditMode.Edit));
					dispatch(recipeActions.setSelectedRecipe(recipe));
					dispatch(editRecipeActions.setRecipe(recipe));

					navigation.dispatch(StackActions.push('Edit Recipe'));
				}}>
					Edit
				</Button>
				<Button onPress={() => {
					dispatch(recipeActions.setSelectedRecipe(recipe));

					navigation.dispatch(StackActions.push('View Recipe'));
				}}>
					View
				</Button>
			</Card.Actions>
		</Card >
	);
}
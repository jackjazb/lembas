import { PayloadAction } from '@reduxjs/toolkit';
import { Ingredient, IngredientQuantityEditable, Recipe, RecipeEditable, toRecipeEditable } from 'lembas-api';
import { createAppSlice } from 'app/redux-utils';

/**
 * Whether we are creating or editing a recipe.
 */
export enum EditMode {
	Create,
	Edit
}
export interface EditRecipeState {
	/** Whether the user is currently editing on creating. */
	editMode: EditMode,
	/** A recipe that is modified as edits occur */
	recipe: RecipeEditable;
	/** An ID in this list will have quantity input errors of some kind. */
	errors: number[];
}

const initialState: EditRecipeState = {
	editMode: EditMode.Create,
	recipe: {
		name: '',
		portions: 2,
		steps: [''],
		ingredients: [],
	},
	errors: []
};

const editRecipeSlice = createAppSlice({
	name: 'editRecipe',
	initialState,
	reducers: (create) => ({
		setEditMode: create.reducer((state, action: PayloadAction<EditMode>) => {
			state.editMode = action.payload;
		}),

		// Override the entire Recipe object state.
		setRecipe: create.reducer((state, action: PayloadAction<Recipe>) => {
			state.errors = [];
			state.recipe = toRecipeEditable(action.payload);
		}),
		clearRecipe: create.reducer((state) => {
			state.recipe = initialState.recipe;
		}),

		// For editing the recipe itself.
		setTitle: create.reducer((state, action: PayloadAction<string>) => {
			state.recipe.name = action.payload;
		}),
		setPortions: create.reducer((state, action: PayloadAction<number>) => {
			state.recipe.portions = action.payload;
		}),
		addStep: create.reducer((state) => {
			state.recipe.steps.push('');
		}),

		// Create a prepared reducer that takes two arguments, which are put into an object before use.
		setStep: create.preparedReducer(
			(index: number, text: string) => ({
				payload: {
					index,
					text
				}
			}),
			(state, action: PayloadAction<{ index: number, text: string; }>) => {
				const index = action.payload.index;
				state.recipe.steps[index] = action.payload.text;
			}
		),
		deleteStep: create.reducer((state, action: PayloadAction<number>) => {
			const index = action.payload;
			state.recipe.steps = state.recipe.steps.filter((s, ci) => index !== ci);
		}),

		addIngredient: create.reducer((state, action: PayloadAction<Ingredient>) => {
			const defaultQuantity: IngredientQuantityEditable = {
				ingredient: action.payload,
				quantity: action.payload.minimum_quantity.toString(),
			};
			state.recipe.ingredients.push(defaultQuantity);
		}),

		setQuantityById: create.reducer((state, action: PayloadAction<{ id: number, quantity: string; }>) => {
			const { id, quantity } = action.payload;
			const index = state.recipe.ingredients.map((i) => i.ingredient.id).indexOf(id);

			// Set the state, but create a validation error if the value is NaN
			if (isNaN(parseFloat(quantity))) {
				state.errors.push(id);
			} else {
				state.errors = state.errors.filter(v => v !== id);
			}

			state.recipe.ingredients[index].quantity = quantity;
		}),

		deleteIngredientById: create.reducer((state, action: PayloadAction<number>) => {
			const id = action.payload;
			state.recipe.ingredients = state.recipe.ingredients.filter((i) => i.ingredient.id !== id);
		}),

	})
});

export const editRecipeActions = editRecipeSlice.actions;
export const editRecipeReducer = editRecipeSlice.reducer;


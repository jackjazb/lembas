import { PayloadAction } from '@reduxjs/toolkit';
import { Recipe, getRecipes } from 'lembas-api';
import { RequestStatus, createAppSlice } from 'app/redux-utils';

export interface RecipeState {
	status: RequestStatus;
	recipes: Recipe[];
	selected?: Recipe,
	error?: string;
}

const initialState: RecipeState = {
	status: RequestStatus.Idle,
	recipes: [],
	selected: undefined,
	error: undefined
};

const recipeSlice = createAppSlice({
	name: 'recipes',
	initialState,
	reducers: (create) => ({
		setSelectedRecipe: create.reducer((state, action: PayloadAction<Recipe>) => {
			state.selected = (action.payload);
		}),
		syncRecipes: create.asyncThunk(
			async (_: void) => {
				const data = await getRecipes();
				return data as Recipe[];
			},
			{
				pending: (state) => {
					state.status = RequestStatus.Loading;
				},
				rejected: (state) => {
					state.status = RequestStatus.Failed;
				},
				fulfilled: (state, action: PayloadAction<Recipe[]>) => {
					state.status = RequestStatus.Succeeded;
					state.recipes = action.payload;
				},
			}

		)
	})

});

export const recipeActions = recipeSlice.actions;
export const recipeReducer = recipeSlice.reducer;


import { PayloadAction } from '@reduxjs/toolkit';
import { Ingredient, searchIngredients } from 'lembas-api';
import { RequestStatus, createAppSlice } from 'app/redux-utils';

export interface IngredientSearchState {
	/** The status of the current request */
	status: RequestStatus;

	/** The most recent request error */
	error?: string;
	/** The current query */
	query: string;

	/** The results of the laste ingredient search */
	results: Ingredient[];

}

const initialState: IngredientSearchState = {
	status: RequestStatus.Idle,
	error: undefined,
	query: '',
	results: [],
};

const ingredientSearchSlice = createAppSlice({
	name: 'ingredientSearch',
	initialState,
	reducers: (create) => ({
		syncResults: create.asyncThunk(
			async (query: string) => {
				const data = await searchIngredients(query);
				return data as Ingredient[];
			},
			{
				pending: (state) => {
					state.status = RequestStatus.Loading;
				},
				rejected: (state) => {
					state.status = RequestStatus.Failed;
				},
				fulfilled: (state, action: PayloadAction<Ingredient[]>) => {
					state.status = RequestStatus.Succeeded;
					state.results = action.payload;
				},
			}

		),
	}),
	selectors: {
		selectSearchStatus: (state) => {
			return state.status;
		},
		selectSearchResults: (state) => {
			return state.results;
		},
	}
});

export const ingredientSearchActions = ingredientSearchSlice.actions;
export const ingredientSearchSelectors = ingredientSearchSlice.selectors;
export const ingredientSearchReducer = ingredientSearchSlice.reducer;


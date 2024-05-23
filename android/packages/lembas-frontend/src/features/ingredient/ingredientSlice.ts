import { PayloadAction } from '@reduxjs/toolkit';
import { Ingredient, IngredientEditable, fromIngredientEditable, getIngredients, getUserIngredients, toIngredientEditable } from 'lembas-api';
import { RequestStatus, createAppSlice } from 'app/redux-utils';

export interface IngredientState {
	/** The status of the current request */
	status: RequestStatus;
	userIngredientStatus: RequestStatus;
	/** All ingredients/user ingredients */
	ingredients: Ingredient[];
	userIngredients: Ingredient[];
	/** The currently selected ingredient */
	selected?: IngredientEditable;
}

const initialState: IngredientState = {
	status: RequestStatus.Idle,
	userIngredientStatus: RequestStatus.Idle,
	ingredients: [],
	userIngredients: [],
	selected: {
		name: '',
		unit: '',
		minimum_quantity: '1',
		purchase_quantity: '1',
		life: '7',
		id: 0,
		user_id: undefined
	},
};

const ingredientSlice = createAppSlice({
	name: 'ingredients',
	initialState,
	reducers: (create) => ({
		setSelectedIngredient: create.reducer<Ingredient>((state, action) => {
			state.selected = toIngredientEditable(action.payload);
		}),
		resetSelectedIngredient: create.reducer(state => {
			state.selected = initialState.selected;
		}),
		setSelectedIngredientName: create.reducer<string>((state, action) => {
			state.selected.name = action.payload;
		}),
		setSelectedIngredientUnit: create.reducer<string>((state, action) => {
			state.selected.unit = action.payload;
		}),
		setSelectedIngredientLife: create.reducer<string>((state, action) => {
			state.selected.life = action.payload;
		}),
		setSelectedIngredientMinPurchase: create.reducer<string>((state, action) => {
			state.selected.purchase_quantity = action.payload;
		}),
		setSelectedIngredientMinQuantity: create.reducer<string>((state, action) => {
			state.selected.minimum_quantity = action.payload;
		}),
		syncIngredients: create.asyncThunk(
			async (_: void) => {
				const data = await getIngredients();
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
					state.ingredients = action.payload;
				},
			}

		),
		syncUserIngredients: create.asyncThunk(
			async (_: void) => {
				const data = await getUserIngredients();
				return data as Ingredient[];
			},
			{
				pending: (state) => {
					state.userIngredientStatus = RequestStatus.Loading;
				},
				rejected: (state) => {
					state.userIngredientStatus = RequestStatus.Failed;
				},
				fulfilled: (state, action: PayloadAction<Ingredient[]>) => {
					state.userIngredientStatus = RequestStatus.Succeeded;
					state.userIngredients = action.payload;
				},
			}

		)
	}),
	selectors: {
		selectIngredientStatus: (state) => {
			return state.status;
		},
		selectIngredients: (state) => {
			return state.ingredients;
		},
		selectCurrentIngredient: (state) => {
			return state.selected;
		},
		selectedIngredientHasErrors: (state) => {
			return !fromIngredientEditable(state.selected);
		}
	}
});

export const ingredientActions = ingredientSlice.actions;
export const ingredientSelectors = ingredientSlice.selectors;
export const ingredientReducer = ingredientSlice.reducer;


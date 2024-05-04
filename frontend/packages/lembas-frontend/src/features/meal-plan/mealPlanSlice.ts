import { PayloadAction } from '@reduxjs/toolkit';
import { Day, ShoppingListEditable, getDays, getISODateString, getShoppingList, } from 'lembas-api';
import { RequestStatus, createAppSlice } from 'app/redux-utils';
import { RootState } from 'app/store';
import { Weekday, addDays, getPreviousDay } from 'features/common/utils';

const RANGE_LENGTH = 7;
const WEEK_START = Weekday.Monday;

export enum EditMode {
	Create,
	Edit
}

/** State for the meal planning view. Note that dates are stored as ISO strings, i.e. "YYYY-MM-DD" */
export interface MealPlanState {
	dayStatus: RequestStatus;
	listStatus: RequestStatus;
	today: string;
	rangeStart: string;
	days: Day[];
	selectedDay?: string;
	shoppingList: ShoppingListEditable;
}

const initialState: MealPlanState = {
	dayStatus: RequestStatus.Idle,
	listStatus: RequestStatus.Idle,
	today: getISODateString(new Date(Date.now())),
	rangeStart: getISODateString(getPreviousDay(new Date(Date.now()), WEEK_START)),
	days: [],
	selectedDay: undefined,
	shoppingList: {
		ingredients: [],
		scheduledIngredients: [],
		checkFor: []
	}
};

const mealPlanSlice = createAppSlice({
	name: 'mealPlan',
	initialState,
	reducers: (create) => ({
		syncDays: create.asyncThunk(
			async (_: void, thunkApi) => {
				const from = (thunkApi.getState() as RootState).mealPlan.rangeStart;
				const to = addDays(from, RANGE_LENGTH - 1);
				const data = await getDays(new Date(from), new Date(to));
				return data as Day[];
			},
			{
				pending: (state) => {
					state.dayStatus = RequestStatus.Loading;
				},
				rejected: (state) => {
					state.dayStatus = RequestStatus.Failed;
				},
				fulfilled: (state, action: PayloadAction<Day[]>) => {
					state.dayStatus = RequestStatus.Succeeded;
					state.days = action.payload;
				},
			}
		),
		syncList: create.asyncThunk(
			async (_: void, thunkApi) => {
				const from = (thunkApi.getState() as RootState).mealPlan.rangeStart;
				const to = addDays(from, RANGE_LENGTH - 1);
				const data = await getShoppingList(new Date(from), new Date(to));
				return data;
			},
			{
				pending: (state) => {
					state.listStatus = RequestStatus.Loading;
				},
				rejected: (state) => {
					state.listStatus = RequestStatus.Failed;
				},
				fulfilled: (state, action: PayloadAction<ShoppingListEditable>) => {
					state.listStatus = RequestStatus.Succeeded;
					state.shoppingList = action.payload;
				},
			}
		),
		// Toggle a list ingredient by type and ID
		toggleListIngredient: create.reducer(
			(state, action: PayloadAction<{ key: keyof ShoppingListEditable, i: number; }>) => {
				const { key, i } = action.payload;
				// Toggle the ingredient at i in the appropriate list :
				state.shoppingList[key][i].ticked = !state.shoppingList[key][i].ticked;
			}
		),
		adjustRange: create.reducer(
			(state, action: PayloadAction<number>) => {
				const currentRangeStart = new Date(state.rangeStart);
				const newRangeStart = new Date(state.rangeStart);
				newRangeStart.setDate(currentRangeStart.getDate() + action.payload);

				state.rangeStart = getISODateString(newRangeStart);
			}
		),
		resetRange: create.reducer(
			(state) => {
				state.rangeStart = initialState.rangeStart;
			}
		),
		setSelectedDay: create.reducer(
			(state, action: PayloadAction<string>) => {
				state.selectedDay = action.payload;
			}
		),
		setListStatus: create.reducer(
			(state, action: PayloadAction<RequestStatus>) => {
				state.listStatus = action.payload;
			}
		),
	}),
	selectors: {
		selectDayStatus: (state) => {
			return state.dayStatus;
		},
		selectListStatus: (state) => {
			return state.dayStatus;
		},
		selectDays: (state) => {
			return state.days;
		},
		selectShoppingList: (state) => {
			return state.shoppingList;
		},
		selectShoppingListEditable: (state) => {
			return state.shoppingList;
		},
		selectCurrentDay: (state) => {
			return state.selectedDay;
		},
		selectToday: (state) => {
			return state.today;
		},
		selectRangeStart: (state) => {
			return state.rangeStart;
		},
		selectRangeEnd: (state) => {
			return addDays(state.rangeStart, RANGE_LENGTH - 1);
		},
		isCurrentWeek: (state) => {
			return state.rangeStart === initialState.rangeStart;
		}
	}
});

export const mealPlanActions = mealPlanSlice.actions;
export const mealPlanSelectors = mealPlanSlice.selectors;
export const mealPlanReducer = mealPlanSlice.reducer;


import { PayloadAction } from '@reduxjs/toolkit';
import { ScheduledIngredient, getScheduledIngredients } from 'lembas-api';
import { RequestStatus, createAppSlice } from 'app/redux-utils';

export interface ScheduledIngredientState {
	/** The status of the current request */
	status: RequestStatus;
	/** The most recent request error */
	error?: string;
	/** The user's ingredient purchase schedule */
	scheduledIngredients: ScheduledIngredient[];
	/** The currently selected scheduled ingredient */
	selected?: ScheduledIngredient;
}

const initialState: ScheduledIngredientState = {
	status: RequestStatus.Idle,
	error: '',
	scheduledIngredients: [],
	selected: undefined,
};

const scheduledIngredientSlice = createAppSlice({
	name: 'scheduledIngredients',
	initialState,
	reducers: (create) => ({
		setSelectedScheduledIngredient: create.reducer<ScheduledIngredient>((state, action) => {
			state.selected = action.payload;
		}),
		setRequestStatus: create.reducer<RequestStatus>((state, action) => {
			state.status = action.payload;
		}),
		syncScheduledIngredients: create.asyncThunk(
			async (_: void) => {
				const data = await getScheduledIngredients();
				return data as ScheduledIngredient[];
			},
			{
				pending: (state) => {
					state.status = RequestStatus.Loading;
				},
				rejected: (state) => {
					state.status = RequestStatus.Failed;
				},
				fulfilled: (state, action: PayloadAction<ScheduledIngredient[]>) => {
					state.status = RequestStatus.Succeeded;
					state.scheduledIngredients = action.payload;
				},
			}

		)
	}),
	selectors: {
		selectIngredientStatus: (state) => {
			return state.status;
		},
		selectScheduledIngredients: (state) => {
			return state.scheduledIngredients;
		},
		selectCurrentScheduledIngredient: (state) => {
			return state.selected;
		}
	}
});

export const scheduledIngredientActions = scheduledIngredientSlice.actions;
export const scheduledIngredientSelectors = scheduledIngredientSlice.selectors;
export const scheduledIngredientReducer = scheduledIngredientSlice.reducer;


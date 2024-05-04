import { buildCreateSlice, asyncThunkCreator } from '@reduxjs/toolkit';

// A reusable type defining the status of an API request.
export enum RequestStatus {
	Idle,
	Loading,
	Succeeded,
	Failed,
}

// Allows using async functions in createSlice
export const createAppSlice = buildCreateSlice({
	creators: { asyncThunk: asyncThunkCreator },
});
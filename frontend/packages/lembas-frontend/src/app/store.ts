import { configureStore } from '@reduxjs/toolkit';
import { ingredientSearchReducer } from 'features/ingredient/ingredientSearchSlice';
import { ingredientReducer } from 'features/ingredient/ingredientSlice';
import { scheduledIngredientReducer } from 'features/ingredient/scheduledIngredientSlice';
import { mealPlanReducer } from 'features/meal-plan/mealPlanSlice';
import { editRecipeReducer } from 'features/recipe/editRecipeSlice';
import { recipeReducer } from 'features/recipe/recipeSlice';

export const store = configureStore({
	reducer: {
		recipes: recipeReducer,
		editRecipe: editRecipeReducer,
		ingredients: ingredientReducer,
		scheduledIngredients: scheduledIngredientReducer,
		ingredientSearch: ingredientSearchReducer,
		mealPlan: mealPlanReducer
	}
});

// Infer the RootState and AppDispatch types from the store
export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
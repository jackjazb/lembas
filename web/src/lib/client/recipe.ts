import type { Ingredient } from "$lib/client/ingredient";
import { del, get } from './utils';
export interface Recipe {
	recipe_id: string,
	title: string,
	portions: number,
	steps: string[];
	ingredients: Ingredient[] | null;
}

export async function getRecipes(): Promise<Recipe[]> {
	return await get('/recipes');
}
export async function getRecipe(id: string): Promise<Recipe> {
	return await get(`/recipes/${id}`);
}
export async function deleteRecipe(id: string): Promise<void> {
	return await del(`/recipes/${id}`);
}
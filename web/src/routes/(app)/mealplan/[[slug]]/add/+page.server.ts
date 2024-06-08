import { getRecipes } from '$lib/client/recipe.js';
import { redirect } from '@sveltejs/kit';

export async function load({ params }) {
	if (!params.slug) {
		redirect(307, "/mealplan");
	}
	const date = new Date(params.slug);
	const recipes = await getRecipes();

	return {
		date,
		recipes
	};
}
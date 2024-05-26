import { getRecipes } from "$lib/client/recipe";

export async function load() {
	return {
		recipes: await getRecipes()
	};
}
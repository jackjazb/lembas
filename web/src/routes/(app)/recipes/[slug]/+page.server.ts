import { getRecipe } from "$lib/client/recipe";

export async function load({ params }) {
	const recipe = await getRecipe(params.slug);

	return {
		recipe
	};
}
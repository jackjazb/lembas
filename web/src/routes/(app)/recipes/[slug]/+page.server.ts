import { getRecipe } from "$lib/client/recipe";

export async function load({ params, url }) {
	const recipe = await getRecipe(params.slug);
	const backUrl = new URLSearchParams(url.search).get('back');

	return {
		recipe,
		backUrl
	};
}
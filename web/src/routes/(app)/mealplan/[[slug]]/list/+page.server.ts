import { getShoppingList } from '$lib/client/list.js';
import { addDays } from '$lib/utils';
import { redirect } from '@sveltejs/kit';

export async function load({ params }) {
	if (!params.slug) {
		redirect(307, '/mealplan');
	}
	const from = new Date(params.slug);
	const to = addDays(from, 6);
	const list = await getShoppingList(from, to);
	return {
		list
	};
}
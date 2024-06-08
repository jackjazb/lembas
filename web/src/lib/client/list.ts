import type { Ingredient } from "$lib/client/ingredient";
import { get } from "$lib/client/utils";
import { toISOString } from "$lib/utils";

export interface IngredientPurchase {
	ingredient: Ingredient,
	units: number;
}

export async function getShoppingList(from: Date, to: Date): Promise<IngredientPurchase[]> {
	const params = {
		from: toISOString(from),
		to: toISOString(to)
	};
	const query = new URLSearchParams(params);
	return await get('/list', query);
}
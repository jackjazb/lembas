export interface Ingredient {
	ingredient_id: number,
	account_id: number | null,
	name: string,
	unit: string | null,
	purchase_unit: number,
	life: number,
	quantity: number;
}

/** Formats an ingredient with its quantity and unit. */
export function formatIngredient(ingredient: Ingredient) {
	if (ingredient.unit) {
		return `${ingredient.quantity}${ingredient.unit} ${ingredient.name}`;
	}
	if (ingredient.quantity > 1) {
		return `${ingredient.quantity} ${ingredient.name}s`;
	}
	return `${ingredient.quantity} ${ingredient.name}`;
}
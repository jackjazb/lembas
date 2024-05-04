import { Ingredient, IngredientEditable, IngredientPurchaseQuantity, IngredientQuantity, Recipe, RecipeEditable, ShoppingList, ShoppingListEditable, ShoppingListItem } from './types';

/**
 * Returns a date into the format YYYY-MM-DD
 */
export function getISODateString(date: Date): string {
	return date.toISOString().split('T')[0];
}

/**
 * Converts a recipe into an editable recipe with all string members.
 */
export function toRecipeEditable(recipe: Recipe): RecipeEditable {
	return {
		...recipe,
		ingredients: recipe.ingredients.map(i => ({
			...i,
			quantity: i.quantity.toString()
		}))
	};
}

/**
 * Try to convert an editable recipe back into it's fully typed equivalent.
 * Returns undefined if the conversion fails.
 */
export function fromRecipeEditable(recipe: RecipeEditable): Recipe | undefined {
	return {
		...recipe,
		ingredients: recipe.ingredients.map(i => ({
			...i,
			quantity: Math.ceil(parseFloat(i.quantity))
		}))
	};
}


/**
 * Converts an ingredient into an editable ingredient with all string members.
 */
export function toIngredientEditable(ingredient: Ingredient): IngredientEditable {
	return {
		...ingredient,
		minimum_quantity: ingredient.minimum_quantity.toString(),
		purchase_quantity: ingredient.purchase_quantity.toString(),
		life: ingredient.life.toString()
	};
}

/**
 * Try to convert an editable ingredient back into it's fully typed equivalent.
 * Returns undefined if the conversion fails.
 */
export function fromIngredientEditable(ingredient: IngredientEditable): Ingredient | undefined {
	if (!ingredient) {
		return undefined;
	}
	if (!ingredient.name) {
		return undefined;
	}
	const minimum_quantity = parseFloat(ingredient.minimum_quantity);
	const purchase_quantity = parseFloat(ingredient.purchase_quantity);
	const life = parseInt(ingredient.life);

	// Ensure all user editable values are valid numbers
	if (!(minimum_quantity && purchase_quantity && life)) {
		return undefined;
	}

	return {
		...ingredient,
		minimum_quantity,
		purchase_quantity,
		life
	};
}

export function toShoppingListEditable(list: ShoppingList): ShoppingListEditable {
	return {
		ingredients: list.ingredients.filter(ing => ing.purchase_quantity > 0)
			.map(ing =>
				toShoppingListItem(ing)
			),
		scheduledIngredients: list.scheduled_ingredients.map(ing =>
			toShoppingListItem(ing)
		),
		checkFor: list.ingredients.filter(ing => ing.purchase_quantity === 0)
			.map(ing =>
				toShoppingListItem(ing, false)
			),
	};
}

export function toShoppingListItem(ingredient: IngredientPurchaseQuantity, ticked: boolean = true): ShoppingListItem {
	return {
		...ingredient,
		ticked
	};
}

/**
 * Converts a shopping list into a combined, usable format.
 */
export function toIngredientQuantityList(shoppingList: ShoppingListEditable): IngredientQuantity[] {
	const ingredients = new Map<number, IngredientQuantity>();
	const allItems = [...shoppingList.ingredients, ...shoppingList.checkFor, ...shoppingList.scheduledIngredients]
		.filter(ing => ing.ticked)
		// If the user overrode any 'check for' items, set their purchase quantity to the minimum
		.map(ing => ing.purchase_quantity === 0 ? { ...ing, purchase_quantity: ing.ingredient.purchase_quantity } : ing);

	for (const item of allItems) {
		const id = item.ingredient.id;
		if (ingredients.get(id)) {
			const quantity = ingredients.get(id).quantity;
			const merged = { ingredient: item.ingredient, quantity: quantity + item.purchase_quantity };
			ingredients.set(id, merged);
		}
		else {
			ingredients.set(id, { ingredient: item.ingredient, quantity: item.purchase_quantity });
		}
	}

	return Array.from(ingredients.values());
}

/**
 * Converts a ShoppingListItem into an IngredientQuantity. Purchase quantity becomes quantity on the output.
*/
export function toIngredientQuantity(shoppingListItem: ShoppingListItem): IngredientQuantity {
	return {
		ingredient: shoppingListItem.ingredient,
		quantity: shoppingListItem.purchase_quantity
	};
}
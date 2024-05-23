export interface Recipe {
	// The recipe's unique ID.
	id?: number;
	// The display name of the recipe.
	name: string;
	// The number of portions this recipe makes.
	portions: number;
	// A JSON array of strings representing the recipe's steps.
	steps: string[];
	// A list of the recipe's ingredients.
	ingredients: IngredientQuantity[];
}

/**
 * These replace the core Recipe and IngredientQuantity types with string versions, 
 * that are validated before conversion to their stored types.
 */
export type IngredientQuantityEditable = Omit<IngredientQuantity, 'quantity'> & { quantity: string; };
export type RecipeEditable = Omit<Recipe, 'ingredients'> & { ingredients: IngredientQuantityEditable[]; };
export type IngredientEditable =
	Omit<Ingredient, 'minimum_quantity' | 'purchase_quantity' | 'life'>
	& {
		minimum_quantity: string;
		purchase_quantity: string;
		life: string;
	};

export interface Ingredient {
	// The ingredient's unique ID
	id: number;
	// Set if this is a custom ingredient
	user_id: number | undefined;
	// The name of the ingredient
	name: string;
	// The unit the above quantity is measured in
	unit: string | undefined;
	// The minimum usable quantity of the ingredient
	minimum_quantity: number;
	// The minimum buyable quantity of the ingredient
	purchase_quantity: number;
	// The number of days the ingredient lasts
	life: number;
}

/**
 * An ingredient in the context of a recipe
 */
export interface IngredientQuantity {
	ingredient: Ingredient;
	quantity: number;
}

/**
 * A reminder to buy an ingredient at a set interval
 */
export interface ScheduledIngredient {
	// The reminder's ID.
	id: number;
	ingredient: Ingredient;
	start_date: string;
	interval: number;
}

/**
 * An ingredient in the context of a shopping list
 */
export interface IngredientPurchaseQuantity {
	ingredient: Ingredient;
	existing_surplus: number;
	used_quantity: number;
	purchase_quantity: number;
}

export interface RecipeInput {
	// The display name of the recipe.
	name: string;
	// The number of portions the recipe makes.
	portions: number;
	// A JSON array of strings representing the recipe's steps.
	steps: string[];
	// Represents the ingredients of the recipe as relations on ID.
	ingredients: RecipeIngredientInput[];
}

/**
 * Represents a single recipe/ingredient relation, with quantity.
 */
export interface RecipeIngredientInput {
	// The ID of the ingredient
	id: number;
	// The amount of the ingredient used in whatever recipe the relation is linked to
	quantity: number;
}

export interface IngredientInput {
	name: string,
	unit: string | undefined,
	minimum_quantity: number,
	purchase_quantity: number,
	life: number,
}

/**
 * The input needed to create an ingredient purchase reminder.
 */
export interface ScheduledIngredientInput {
	ingredient_id: number,
	start_date: string,
	interval: number,
}

/**
 * Represents a calendar day with a set of recipes.
 */
export interface Day {
	date: string;
	recipes: Recipe[];
}

/**
 * The input needed to add a recipe to a day.
 */
export interface DayInput {
	recipe_id: number;
	date: string;
}

/**
 * Represents a shoppings list for a given range.
 */
export interface ShoppingList {
	ingredients: IngredientPurchaseQuantity[];
	scheduled_ingredients: IngredientPurchaseQuantity[];
}

interface ShoppingListAttributes {
	ticked: boolean;
}

export type ShoppingListItem = IngredientPurchaseQuantity & ShoppingListAttributes;

export interface ShoppingListEditable {
	ingredients: ShoppingListItem[];
	scheduledIngredients: ShoppingListItem[];
	checkFor: ShoppingListItem[];
}

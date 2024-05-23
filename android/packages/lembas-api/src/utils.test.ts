import { describe, expect, it } from '@jest/globals';
import { fromIngredientEditable, fromRecipeEditable, getISODateString, toIngredientEditable, toIngredientQuantity, toIngredientQuantityList, toRecipeEditable, toShoppingListEditable, toShoppingListItem } from './utils';
import { Ingredient, IngredientEditable, IngredientPurchaseQuantity, IngredientQuantity, IngredientQuantityEditable, Recipe, RecipeEditable, ShoppingList, ShoppingListEditable, ShoppingListItem } from './types';

// Define some test ingredients
const ingredient: Ingredient = {
	id: 1,
	user_id: undefined,
	name: 'Test Ingredient',
	unit: 'g',
	minimum_quantity: 50,
	purchase_quantity: 50,
	life: 7,
};

const ingredientEditable: IngredientEditable = {
	id: 1,
	user_id: undefined,
	name: 'Test Ingredient',
	unit: 'g',
	minimum_quantity: '50',
	purchase_quantity: '50',
	life: '7',
};

const ingredientQuantity: IngredientQuantity = {
	ingredient: ingredient,
	quantity: 50
};

const ingredientPurchaseQuantity: IngredientPurchaseQuantity = {
	ingredient,
	existing_surplus: 5,
	purchase_quantity: 10,
	used_quantity: 10
};

const ingredientQuantityEditable: IngredientQuantityEditable = {
	ingredient: ingredient,
	quantity: '50'
};

describe('getISODateString', () => {
	it('should parse a date object into a string of the correct format', () => {
		const date = new Date('05 June 2023 14:48 UTC');
		const result = getISODateString(date);
		expect(result).toEqual('2023-06-05');
	});
});

const recipe: Recipe = {
	name: 'Test Recipe',
	portions: 4,
	steps: ['Step 1', 'Step 2'],
	ingredients: [ingredientQuantity],
};

const recipeEditable: RecipeEditable = {
	name: 'Test Recipe',
	portions: 4,
	steps: ['Step 1', 'Step 2'],
	ingredients: [ingredientQuantityEditable],
};

describe('toRecipeEditable', () => {
	it('should convert necessary recipe fields to strings', () => {
		const recipeEditable = toRecipeEditable(recipe);
		expect(recipeEditable).toStrictEqual(recipeEditable);
	});
});

describe('fromRecipeEditable', () => {
	it('should convert an editable recipe to read-only', () => {
		const recipe = fromRecipeEditable(recipeEditable);
		expect(recipe).toStrictEqual(recipe);
	});

	it('should return undefined for non-parseable recipes', () => {
		const badRecipe = {
			...recipeEditable,
			ingredients: [{
				ingredient: ingredient,
				quantity: 'something else'
			}],
		};
		const recipe = fromRecipeEditable(badRecipe);
		expect(recipe).toBeUndefined;
	});
});

describe('toIngredientEditable', () => {
	it('should convert necessary ingredient fields to strings', () => {
		const result = toIngredientEditable(ingredient);
		expect(result).toStrictEqual(ingredientEditable);
	});
});

describe('fromIngredientEditable', () => {
	it('should convert an editable ingredient to read-only', () => {
		const result = fromIngredientEditable(ingredientEditable);
		expect(result).toStrictEqual(ingredient);
	});
	it('should return undefined in passed an undefined ingredient', () => {
		const result = fromIngredientEditable(undefined);
		expect(result).toStrictEqual(undefined);
	});

	it('should return undefined if name is empty', () => {
		const badIngredient: IngredientEditable = {
			...ingredientEditable,
			name: undefined
		};
		const result = fromIngredientEditable(badIngredient);
		expect(result).toBeUndefined;
	});

	it('should return undefined if quantities are not parseable', () => {
		const badIngredient: IngredientEditable = {
			...ingredientEditable,
			minimum_quantity: 'bad input',
			purchase_quantity: 'bad input'
		};
		const result = fromIngredientEditable(badIngredient);
		expect(result).toBeUndefined;
	});

	it('should return undefined if life is not parseable', () => {
		const badIngredient: IngredientEditable = {
			...ingredientEditable,
			life: 'bad input'
		};
		const result = fromIngredientEditable(badIngredient);
		expect(result).toBeUndefined;
	});
});


describe('toShoppingListEditable', () => {

	const shoppingList: ShoppingList = {
		ingredients: [
			ingredientPurchaseQuantity
		],
		scheduled_ingredients: [
			ingredientPurchaseQuantity
		]
	};

	const shoppingListItem: ShoppingListItem = {
		ingredient,
		existing_surplus: 5,
		purchase_quantity: 10,
		used_quantity: 10,
		ticked: true
	};

	const shoppingListEditable: ShoppingListEditable = {
		ingredients: [shoppingListItem],
		scheduledIngredients: [shoppingListItem],
		checkFor: [],

	};

	it('should convert list ingredients to shopping list items', () => {
		const result = toShoppingListEditable(shoppingList);
		expect(result).toStrictEqual(shoppingListEditable);
	});

	it('should place zero purchase required ingredients in "Check For"', () => {
		const noPurchase = {
			...ingredientPurchaseQuantity,
			purchase_quantity: 0
		};
		const listWithSurplus = {
			...shoppingList,
			ingredients: [noPurchase]
		};
		const result = toShoppingListEditable(listWithSurplus);
		expect(result).toStrictEqual({
			ingredients: [],
			scheduledIngredients: [{ ...shoppingListItem, ticked: true }],
			checkFor: [{ ...shoppingListItem, purchase_quantity: 0, ticked: false }],
		});
	});
});

describe('toShoppingListItem', () => {
	it('should add a "ticked" field to ingredients', () => {
		const result = toShoppingListItem(ingredientPurchaseQuantity, false);
		expect(result).toStrictEqual({ ...ingredientPurchaseQuantity, ticked: false });
	});
});

describe('toIngredientQuantityList', () => {
	it('should filter out unticked items', () => {
		const detailedList: ShoppingListEditable = {
			ingredients: [{
				ingredient: {
					id: 1,
					user_id: undefined,
					name: 'Test Ingredient',
					unit: 'g',
					minimum_quantity: 50,
					purchase_quantity: 50,
					life: 7,
				},
				existing_surplus: 5,
				purchase_quantity: 100,
				used_quantity: 60,
				ticked: true
			},
			{
				ingredient: {
					id: 2,
					user_id: undefined,
					name: 'Test Ingredient',
					unit: 'g',
					minimum_quantity: 50,
					purchase_quantity: 50,
					life: 7,
				},
				existing_surplus: 5,
				purchase_quantity: 100,
				used_quantity: 60,
				ticked: false
			}],
			scheduledIngredients: [],
			checkFor: []
		};
		const result = toIngredientQuantityList(detailedList);
		expect(result.length).toEqual(1);
		expect(result[0].ingredient.id).toEqual(1);
		expect(result[0].quantity).toEqual(100);

	});

	it('should scale up quantity of ticked "check for" items', () => {
		const detailedList: ShoppingListEditable = {
			ingredients: [],
			scheduledIngredients: [],
			checkFor: [{
				ingredient: {
					id: 5,
					user_id: undefined,
					name: 'Test Ingredient',
					unit: 'g',
					minimum_quantity: 50,
					purchase_quantity: 50,
					life: 7,
				},
				existing_surplus: 5,
				purchase_quantity: 0,	// This value scaled up.
				used_quantity: 60,
				ticked: true
			}]
		};
		const result = toIngredientQuantityList(detailedList);
		expect(result[0].quantity).toEqual(50);
	});

	it('should merge duplicate ingredients', () => {
		const detailedList: ShoppingListEditable = {
			ingredients: [{
				ingredient: {
					id: 5,
					user_id: undefined,
					name: 'Test Ingredient',
					unit: 'g',
					minimum_quantity: 50,
					purchase_quantity: 50,
					life: 7,
				},
				existing_surplus: 5,
				purchase_quantity: 0,
				used_quantity: 60,
				ticked: true
			}],
			scheduledIngredients: [{
				ingredient: {
					id: 5,
					user_id: undefined,
					name: 'Test Ingredient',
					unit: 'g',
					minimum_quantity: 50,
					purchase_quantity: 50,
					life: 7,
				},
				existing_surplus: 5,
				purchase_quantity: 50,
				used_quantity: 60,
				ticked: true
			}],
			checkFor: []
		};
		const result = toIngredientQuantityList(detailedList);
		expect(result[0].quantity).toEqual(100);
	});
});

describe('toIngredientQuantity', () => {
	it('should convert shopping list items to ingredient quantities', () => {
		const item: ShoppingListItem = {
			ingredient,
			existing_surplus: 5,
			purchase_quantity: 50,
			used_quantity: 10,
			ticked: true
		};

		const result = toIngredientQuantity(item);
		expect(result).toStrictEqual({
			ingredient,
			quantity: 50
		});
	});
});
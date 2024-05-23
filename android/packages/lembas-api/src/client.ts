import { RecipeInput, Ingredient, RecipeIngredientInput, Recipe, DayInput, Day, ShoppingList, ScheduledIngredient, ScheduledIngredientInput, IngredientInput, ShoppingListEditable } from './types';
import { getISODateString, toShoppingListEditable } from './utils';
import { fetchAuthSession } from 'aws-amplify/auth';

export const API_URL = 'https://api.lmbas.net';

/**
 * Test that the API route returns an HTTP 200
 */
export async function connectionTest(): Promise<boolean> {
	try {
		const res = await get('');
		return res.status === 200;
	}
	catch (e) {
		console.log('Failed to reach server', e);
		return false;
	}
}


/** Creates some example data under the current user.*/
export async function loadDemoData() {
	const sourdough: RecipeInput = {
		name: 'Sourdough',
		portions: 2,
		ingredients: [
			{
				// Flour
				id: 940,
				quantity: 500
			},
			{
				// Water
				id: 1156,
				quantity: 375
			},
			{
				// Salt
				id: 957,
				quantity: 10
			}
		],
		steps: [
			'Combine dough ingredients in a large bowl',
			'Mix by hand until combined',
			'Leave to autolyse, performing three sets of stretch and folds at 15 minute intervals',
			'Leave to ferment for 1 hour',
			'Shape and prove for 1 more hour',
			'Preheat a dutch oven to 250C. Bake for 20 minutes covered, then uncovered until dark brown.',
		]
	};
	const celeriac_soup: RecipeInput = {
		name: 'Celeriac Soup',
		portions: 4,
		ingredients: [
			{
				// Celeriac
				id: 653,
				quantity: 1
			},
			{
				// Potato
				id: 476,
				quantity: 1
			},
			{
				// Onion
				id: 471,
				quantity: 1
			},
			{
				// Stock
				id: 1083,
				quantity: 1000
			},
			{
				// Thyme
				id: 811,
				quantity: 5
			},
			{
				// Garlic
				id: 487,
				quantity: 1
			},
			{
				// Oat Drink
				id: 329,
				quantity: 50
			}
		],
		steps: [
			'Tie thyme in a bunch and lightly brown in oil',
			'Add onions and sautee for a few minutes',
			'Add celeriac, potato and stock and simmer until soft',
			'Blend until smooth'
		]
	};
	const lentil_pasta: RecipeInput = {
		name: 'Lentil Pasta',
		portions: 4,
		ingredients: [
			{
				// Lentils
				id: 924,
				quantity: 500
			},
			{
				// Chopped tomatoes
				id: 1144,
				quantity: 227
			},
			{
				// Basil
				id: 513,
				quantity: 5
			},
			{
				// Oregano
				id: 739,
				quantity: 5
			},
			{
				// Stock
				id: 1083,
				quantity: 500
			},
			{
				// Garlic
				id: 487,
				quantity: 1
			},
			{
				// Wholewheat Spaghetti
				id: 1032,
				quantity: 200
			}
		],
		steps: [
			'Sautee carrots until softened',
			'Add lentils and brown',
			'Add remaining ingredients and simmer until lentils are soft',
			'Meanwhile, cook pasta until al dente. Combine and serve',
		]
	};

	const coffee = {
		name: 'Coffee',
		life: 30,
		minimum_quantity: 1,
		purchase_quantity: 300,
		unit: 'g'
	};
	const rolled_oats = {
		name: 'Rolled Oats',
		life: 365,
		minimum_quantity: 1,
		purchase_quantity: 500,
		unit: 'g'
	};

	try {
		await post('recipes', JSON.stringify(sourdough));
		await post('recipes', JSON.stringify(celeriac_soup));
		await post('recipes', JSON.stringify(lentil_pasta));
		await post('ingredients', JSON.stringify(coffee));
		await post('ingredients', JSON.stringify(rolled_oats));
	}
	catch (e) {
		console.log('Failed to create recipe:', e);
		return Promise.reject('Failed to create recipe:');
	}
}

/**
 * Fetch all recipes for a given user.
 * @param userId The current user
 * @returns 
 */
export async function getRecipes(): Promise<Recipe[]> {
	try {
		const response = await get('recipes');
		const recipes = await response.json() as Recipe[];
		return recipes;
	}
	catch (e) {
		console.log('Failed to fetch recipes:', e);
		return Promise.reject();
	}
}

export async function createRecipe(recipe: Recipe): Promise<number> {
	// Map the Recipe object onto a RecipeInput
	const ingredientInputs: RecipeIngredientInput[] = recipe.ingredients.map(ingredient => ({
		id: ingredient.ingredient.id,
		quantity: ingredient.quantity
	}));

	const recipeInput: RecipeInput = {
		ingredients: ingredientInputs,
		name: recipe.name,
		steps: recipe.steps,
		portions: recipe.portions
	};

	try {
		const res = await post('recipes', JSON.stringify(recipeInput));
		return res.status;
	}
	catch (e) {
		console.log('Failed to create recipe:', e);
		return Promise.reject('Failed to create recipe:');
	}
}


export async function updateRecipe(recipe: Recipe): Promise<number> {
	// Map the Recipe object onto a RecipeInput
	const ingredientInputs: RecipeIngredientInput[] = recipe.ingredients.map(ingredient => ({
		id: ingredient.ingredient.id,
		quantity: ingredient.quantity
	}));

	const recipeInput: RecipeInput = {
		ingredients: ingredientInputs,
		name: recipe.name,
		steps: recipe.steps,
		portions: recipe.portions
	};
	try {
		const res = await put(`recipe/${recipe.id}`, JSON.stringify(recipeInput));
		return res.status;
	}
	catch (e) {
		console.log('Failed to update recipe:', e);
		return Promise.reject();
	}

}

export async function deleteRecipe(recipeId: number): Promise<number> {
	try {
		const res = await del(`recipe/${recipeId}`);
		return res.status;
	}
	catch (e) {
		console.log('Failed to delete recipe:', e);
		return Promise.reject();
	}
}

export async function getShoppingList(from: Date, to: Date): Promise<ShoppingListEditable> {
	try {
		const response = await get(`shoppinglist?from=${getISODateString(from)}&to=${getISODateString(to)}`);
		const list = await response.json() as ShoppingList;
		return toShoppingListEditable(list);
	}
	catch (e) {
		console.log('Failed to fetch shopping list:', e);
		return Promise.reject();
	}
}
export async function getDays(from: Date, to: Date): Promise<Day[]> {
	try {
		const response = await get(`days?from=${getISODateString(from)}&to=${getISODateString(to)}`);
		const days = await response.json() as Day[];
		return days;
	}
	catch (e) {
		console.log('Failed to fetch days:', e);
		return Promise.reject();
	}
}

/**
 * Creates a new date/recipe relation for meal planning 
 */
export async function createDay(recipe: Recipe, date: string): Promise<number> {
	const input: DayInput = { recipe_id: recipe.id, date };
	try {
		const res = await post('days', JSON.stringify(input));
		return res.status;
	}
	catch (e) {
		console.log('Failed to create day', e);
		return Promise.reject();
	}
}

/**
 * Deletes a day from the database 
 */
export async function deleteRecipeFromDay(recipe: Recipe, date: Date): Promise<number> {
	try {
		const res = await del(`days/${getISODateString(date)}/recipes/${recipe.id}`);
		return res.status;
	}
	catch (e) {
		console.log('Failed to delete recipe from day:', e);
		return Promise.reject();
	}
}


/**
 * Fetches all ingredients
 */
export async function getIngredients(): Promise<Ingredient[]> {
	try {
		const response = await get('ingredients');
		const ingredients = await response.json() as Ingredient[];
		return ingredients;
	}
	catch (e) {
		console.log('Failed to fetch ingredients:', e);
		return Promise.reject();
	}
}


/**
 * Fetches user ingredients
 */
export async function getUserIngredients(): Promise<Ingredient[]> {
	try {
		const response = await get('ingredients/user');
		const ingredients = await response.json() as Ingredient[];
		return ingredients;
	}
	catch (e) {
		console.log('Failed to fetch ingredients:', e);
		return Promise.reject();
	}
}

/**
 * Fetches all scheduled ingredients
 */
export async function getScheduledIngredients(): Promise<ScheduledIngredient[]> {
	try {
		const response = await get('schedule');
		const ingredients = await response.json() as ScheduledIngredient[];
		return ingredients;
	}
	catch (e) {
		console.log('Failed to fetch schedule:', e);
		return Promise.reject();
	}
}


/**
 * Creates a scheduled ingredient.
 */
export async function createScheduledIngredient(ingredient: Ingredient, startDate: string, interval: number): Promise<number> {
	const input: ScheduledIngredientInput = {
		ingredient_id: ingredient.id,
		start_date: startDate,
		interval
	};
	try {
		const res = await post('schedule', JSON.stringify(input));
		return res.status;
	}
	catch (e) {
		console.log('Failed to create scheduled ingredient', e);
		return Promise.reject();
	}
}

/**
 * Updates a scheduled ingredient for the current user.
 */
export async function createUserIngredient(ingredient: Ingredient): Promise<number> {

	const input: IngredientInput = {
		name: ingredient.name,
		life: ingredient.life,
		minimum_quantity: ingredient.minimum_quantity,
		purchase_quantity: ingredient.purchase_quantity,
		unit: ingredient.unit === '' ? undefined : ingredient.unit
	};
	try {
		const res = await post('ingredients', JSON.stringify(input));
		return res.status;
	}
	catch (e) {
		console.log('Failed to update ingredient', e);
		return Promise.reject();
	}
}


/**
 * Updates a scheduled ingredient for the current user.
 */
export async function deleteUserIngredient(ingredientId: number): Promise<number> {

	try {
		const res = await del(`ingredient/${ingredientId}`);
		return res.status;
	}
	catch (e) {
		console.log('Failed to update ingredient', e);
		return Promise.reject();
	}
}

/**
 * Deletes a scheduled ingredient.
 */
export async function deleteScheduledIngredient(ingredient: ScheduledIngredient): Promise<number> {

	try {
		const res = await del(`schedule/${ingredient.id}`);
		return res.status;
	}
	catch (e) {
		console.log('Failed to delete scheduled ingredient', e);
		return Promise.reject();
	}
}

/**
 * Performs a search over the ingredients table.
 * @param query The string to search for
 * @returns 
 */
export async function searchIngredients(query: string): Promise<Ingredient[]> {
	if (query.length === 0) {
		return [];
	}
	try {
		const response = await get(`search/ingredients?query=${query}`);
		const ingredients = await response.json() as Ingredient[];
		return ingredients;
	}
	catch (e) {
		console.log('Failed to perform ingredient search', e);
		return Promise.reject();
	}
}

/**
 * Send a GET request to the API.
 * 
 * @param uri The URI to fetch.
 */
async function get(uri: string) {
	return await fetch(`${API_URL}/${uri}`, {
		headers: {
			'Authorization': await resolveAuthHeader(),
			'Accept': 'application/json'
		}
	});
}

/**
 * Send a POST request to the API.
 * 
 * @param uri The URI to POST.
 * @param body The body of the request - should be stringified JSON.
 */
async function post(uri: string, body: string) {
	return await fetch(`${API_URL}/${uri}`, {
		method: 'POST',
		headers: {
			'Authorization': await resolveAuthHeader(),
			'Accept': 'application/json',
			'Content-Type': 'application/json',
		},
		body
	});
}


/**
 * Send a PUT request to the API.
 * 
 * @param uri The URI to POST.
 * @param body The body of the request - should be stringified JSON.
 */
async function put(uri: string, body: string) {
	return await fetch(`${API_URL}/${uri}`, {
		method: 'PUT',
		headers: {
			'Authorization': await resolveAuthHeader(),
			'Accept': 'application/json',
			'Content-Type': 'application/json',
		},
		body
	});
}

/**
 * Send a DELETE request to the API.
 * 
 * @param uri The resource to delete.
 */
async function del(uri: string) {
	return await fetch(`${API_URL}/${uri}`, {
		method: 'DELETE',
		headers: {
			'Authorization': await resolveAuthHeader(),
			'Accept': 'application/json'
		}
	});
}

export async function resolveAuthHeader() {
	const { tokens } = await fetchAuthSession();
	const token = tokens.accessToken.toString();
	return `Bearer ${token}`;
} 
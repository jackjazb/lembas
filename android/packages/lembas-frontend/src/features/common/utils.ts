import { Ingredient, IngredientQuantity, Recipe, getISODateString } from 'lembas-api';
import { ToastAndroid } from 'react-native';

/**
 * Utility function to remove an item from an array at index `i`
 * @param array The array to remove the item from.
 * @param index The index of the item to remove.
 * @returns A modified array.
 */
export function removeByIndex(array, index: number) {
	return array.filter((item, i) => i !== index);
}

/**
 * Format an ingredient with quantity and unit
 * @param ingredient The ingredient to format
 */
export function formatIngredientQuantity(ingredient: IngredientQuantity) {
	const unitString = getUnitString(ingredient.ingredient);
	return `${ingredient.quantity} ${unitString} `;
}

/**
 * Adds an 's' to an ingredient's name if it has no unit and quantity greater than 1.
 * @param ingredient The ingredient to format
 */
export function formatIngredientName(ingredient: IngredientQuantity) {
	let { name } = ingredient.ingredient;
	if (!ingredient.ingredient.unit && ingredient.quantity > 1) {
		name = name + 's';
	}
	return name;
}
/**
 * Formats a unit in a more readable way.
 */
export function getUnitString(ingredient: Ingredient) {
	const { unit } = ingredient;
	if (!unit) {
		return '';
	}
	// Padd anything bigger than 2 characters - looks less messy for word-based units.
	if (unit.length > 2) {
		return ` ${unit}`;
	}
	return unit;
}

/**
 * Formats a date as "Day, Month 1st"
 */
export function formatDateWithDay(date: Date): string {
	const weekday = getWeekday(date);
	const day = date.getDate() + getSuffix(date.getDate());
	const month = date.toLocaleString('default', { month: 'long' });

	return `${weekday}, ${month} ${day}`;
}

export function formatDate(date: Date) {
	const day = date.getDate() + getSuffix(date.getDate());
	const month = date.toLocaleString('default', { month: 'long' });

	return `${month} ${day}`;
}
/**
 * Returns the day of the week of a given date.
 */
export function getWeekday(date: Date): string {
	const weekdays = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
	return weekdays[date.getDay()];
}

/**
 * Returns an appropriate suffix for a number.
 */
export function getSuffix(number: number) {
	if (number > 3 && number < 21) return 'th';
	switch (number % 10) {
		case 1:
			return 'st';
		case 2:
			return 'nd';
		case 3:
			return 'rd';
		default:
			return 'th';
	}
}

const DAY_MS = 1000 * 60 * 60 * 24;
/**
 * Calculate the next day that occurs a multiple of the interval from the start date. 
 */
export function getNextIntervalDate(start: Date, interval: number) {
	const initial = new Date(Date.now());
	const daysSinceStart = Math.max(Math.floor((initial.getTime() - start.getTime()) / DAY_MS), interval);

	const nextOccurenceDays = Math.ceil(daysSinceStart / interval) * interval;
	return new Date(start.getTime() + nextOccurenceDays * DAY_MS);
}

/** Show a toast message with a given text. */
export function showToast(text: string) {
	ToastAndroid.show(text, ToastAndroid.SHORT);
}

/** A day of the week */
export enum Weekday {
	Sunday,
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday
}

/**
 * Given a date and day index (0-6), finds the date of the most recent occurrence of that day. Sunday is 0.
 */
export function getPreviousDay(date: Date, day: Weekday) {
	const currentDay = date.getDay();
	let diff = currentDay - day;
	if (diff < 0) {
		diff = 7 + diff;
	}
	return new Date(date.setDate(date.getDate() - diff));

}

/**
 * Adds a number of days to a date represented as an ISO date string.
 */
export function addDays(date: string, days: number) {
	const endDate = new Date(date);
	endDate.setDate(new Date(date).getDate() + days);
	return getISODateString(endDate);

}

export function scaleToPortions(recipe: Recipe, portions: number): [Recipe, boolean] {
	const scaleFactor = portions / recipe.portions;
	const scaledRecipe: Recipe = { ...recipe, portions, ingredients: [] };
	let subMinimumQuantity = false;
	for (const ingredient of recipe.ingredients) {
		const scaledIngredient: IngredientQuantity = {
			...ingredient,
			quantity: ingredient.quantity * scaleFactor
		};
		if (scaledIngredient.quantity < ingredient.ingredient.minimum_quantity) {
			subMinimumQuantity = true;
		}
		scaledRecipe.ingredients.push(scaledIngredient);
	}
	return [scaledRecipe, subMinimumQuantity];
}

export function getPlaintextList(ingredients: IngredientQuantity[]): string {
	let text = 'Shopping List\n\n';
	for (const ingredient of ingredients) {
		text += `- ${ingredient.ingredient.name}, ${ingredient.quantity}${getUnitString(ingredient.ingredient)}\n`;
	}
	return text;
}
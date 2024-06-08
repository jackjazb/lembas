import type { Recipe } from "$lib/client/recipe";
import { del, get, post } from "$lib/client/utils";
import { toISOString } from "$lib/utils";

export interface Day {
	day_id: string,
	recipe: Recipe,
	date: string;
}

export interface DayInput {
	recipe_id: string,
	date: string;
}


export async function getDays(from: Date, to: Date): Promise<Day[]> {
	const params = {
		from: toISOString(from),
		to: toISOString(to)
	};
	const query = new URLSearchParams(params);
	return await get('/days', query);
}

export async function createDay(recipe: Recipe, date: Date): Promise<void> {
	const payload: DayInput = {
		recipe_id: recipe.recipe_id,
		date: toISOString(date)
	};
	await post('/days', JSON.stringify(payload));
}

export async function deleteDay(id: string): Promise<void> {
	await del(`/days/${id}`);
}

export function groupByDate(days: Day[]): Record<string, Day[]> {
	const grouped: Record<string, Day[]> = {};
	for (const day of days) {
		const key = day.date;
		if (!grouped[key]) {
			grouped[key] = [];
		}
		grouped[key].push(day);
	}
	return grouped;
}
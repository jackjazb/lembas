import { groupByDate, type Day } from "$lib/client/day";
import type { Recipe } from "$lib/client/recipe";
import { describe, expect, it } from "vitest";

describe('groupByDate', () => {

	it('should group a list of days by their date', () => {
		const recipe: Recipe = {
			recipe_id: "1",
			title: "Recipe",
			portions: 0,
			steps: [],
			ingredients: null
		};

		const days: Day[] = [{
			day_id: "1",
			recipe,
			date: '2020-01-01'
		}, {
			day_id: "2",
			recipe,
			date: '2020-01-01'
		},
		{
			day_id: "3",
			recipe,
			date: '2020-01-02'
		}];

		const expected = {
			"2020-01-01": [
				{
					day_id: "1",
					recipe,
					date: '2020-01-01'
				}, {
					day_id: "2",
					recipe,
					date: '2020-01-01'
				},
			],
			"2020-01-02": [
				{
					day_id: "3",
					recipe,
					date: '2020-01-02'
				}
			]
		};

		const grouped = groupByDate(days);
		expect(grouped).toEqual(expected);
	});
});
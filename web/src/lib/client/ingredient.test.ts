import { formatIngredient, type Ingredient } from '$lib/client/ingredient';
import { describe, it, expect } from 'vitest';

describe('formatIngredient', () => {
	it('should append "s" to ingredients if unit is undefined', () => {
		const ingredient: Ingredient = {
			ingredient_id: 1,
			account_id: undefined,
			name: "Apple",
			unit: undefined,
			purchase_unit: 6,
			life: 7,
			quantity: 5
		};
		expect(formatIngredient(ingredient)).toEqual('5 Apples');
	});

	it('should not output a unit if unit is undefined', () => {
		const ingredient: Ingredient = {
			ingredient_id: 1,
			account_id: undefined,
			name: "Apple",
			unit: undefined,
			purchase_unit: 6,
			life: 7,
			quantity: 1
		};
		expect(formatIngredient(ingredient)).toEqual('1 Apple');
	});

	it('should output a unit if present', () => {
		const ingredient: Ingredient = {
			ingredient_id: 1,
			account_id: undefined,
			name: "Flour",
			unit: "g",
			purchase_unit: 500,
			life: 7,
			quantity: 400
		};
		expect(formatIngredient(ingredient)).toEqual('400g Flour');
	});
});

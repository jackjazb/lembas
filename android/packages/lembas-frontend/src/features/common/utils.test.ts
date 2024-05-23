import { describe, it, expect, jest } from '@jest/globals';
import { formatDate, formatDateWithDay, formatIngredientQuantity, getPlaintextList, getSuffix, getWeekday, removeByIndex } from 'features/common/utils';
import { Ingredient, IngredientQuantity } from 'lembas-api';

jest.mock('lembas-api', () => {
	return {};
});

describe('removeByIndex', () => {
	it('should remove an item from an array the specified index', () => {
		const list = [1, 2, 3];
		const result = removeByIndex(list, 1);
		expect(result).toStrictEqual([1, 3]);
	});
});

describe('formatIngredientQuantity', () => {
	it('should correctly format an ingredient quantity', () => {
		const ingredient: Ingredient = {
			id: 1,
			user_id: undefined,
			name: 'Test Ingredient',
			unit: 'g',
			minimum_quantity: 50,
			purchase_quantity: 50,
			life: 7,
		};

		const ingredientQuantity = {
			ingredient: ingredient,
			quantity: 50,
		};

		const result = formatIngredientQuantity(ingredientQuantity);
		expect(result).toEqual('50 g ');
	});

	it('should correctly format an ingredient with a null unit', () => {
		const ingredient: Ingredient = {
			id: 1,
			user_id: undefined,
			name: 'Test Ingredient',
			unit: null,
			minimum_quantity: 50,
			purchase_quantity: 50,
			life: 7,
		};

		const ingredientQuantity = {
			ingredient: ingredient,
			quantity: 50,
		};

		const result = formatIngredientQuantity(ingredientQuantity);
		expect(result).toEqual('50  ');
	});
});

describe('formatDateWithDay', () => {
	it('should format dates as "Day, Month nth"', () => {
		const date = new Date('2023-01-01');
		const result = formatDateWithDay(date);
		expect(result).toEqual('Sunday, January 1st');
	});
});

describe('formatDate', () => {
	it('should format dates as "Month nth"', () => {
		const date = new Date('2023-01-01');
		const result = formatDate(date);
		expect(result).toEqual('January 1st');
	});
});

describe('getWeekday', () => {
	it('should identify the day of the week for a given date', () => {
		const date = new Date('2023-01-01');
		const result = getWeekday(date);
		expect(result).toEqual('Sunday');
	});
});

describe('getSuffix', () => {
	it('should append "st" to 1', () => {
		expect(getSuffix(1)).toEqual('st');
	});
	it('should append "nd" to 2', () => {
		expect(getSuffix(2)).toEqual('nd');
	});
	it('should append "rd" to 3', () => {
		expect(getSuffix(3)).toEqual('rd');
	});
	it('should append "th" in all other cases', () => {
		expect(getSuffix(4)).toEqual('th');
	});
});

describe('getPlaintextList', () => {
	it('should convert a shopping list to plaintext', () => {
		const ingredients: IngredientQuantity[] = [{
			ingredient: {
				id: 5,
				user_id: undefined,
				name: 'Oat Drink',
				unit: 'ml',
				minimum_quantity: 50,
				purchase_quantity: 50,
				life: 7,
			},
			quantity: 50,
		}, {
			ingredient: {
				id: 5,
				user_id: undefined,
				name: 'Flour',
				unit: 'g',
				minimum_quantity: 50,
				purchase_quantity: 50,
				life: 7,
			},
			quantity: 500,
		}, {
			ingredient: {
				id: 5,
				user_id: undefined,
				name: 'Oats',
				unit: 'g',
				minimum_quantity: 50,
				purchase_quantity: 50,
				life: 7,
			},
			quantity: 400,
		}];

		const text = getPlaintextList(ingredients);
		expect(text).toEqual('Shopping List\n\n- Oat Drink, 50ml\n- Flour, 500g\n- Oats, 400g\n');
	});
});
import { addDays, Weekday, getDayAbbr, getStartOfWeek, toISOString, sortISOStrings } from "$lib/utils";
import { describe, expect, it } from "vitest";

describe('toISOString', () => {
	it('should format a date correctly', () => {
		const date = new Date('2020-01-01');
		expect(toISOString(date)).toEqual('2020-01-01');
	});
});

describe('getDayAbbr', () => {
	it('should return the correct abbreviation for a given date', () => {
		const date = new Date('2024-06-02');
		expect(getDayAbbr(date)).toEqual('SUN');
	});
});

describe('addDays', () => {
	it('should return a new date object with days added', () => {
		const date = new Date('2024-06-02');
		const laterDate = new Date('2024-06-07');
		expect(addDays(date, 5)).toEqual(laterDate);
	});
});

describe('getStartOfWeek', () => {
	it('should return the date of the first day of the week', () => {
		const date = new Date('2024-06-02');
		const startOfWeek = new Date('2024-05-27');
		expect(getStartOfWeek(date)).toEqual(startOfWeek);
	});
	it('should allow the start of the week to be set', () => {
		const date = new Date('2024-06-02');
		const firstDay = Weekday.Tuesday;
		const startOfWeek = new Date('2024-05-28');
		expect(getStartOfWeek(date, firstDay)).toEqual(startOfWeek);
	});
});

describe('sortISOStrings', () => {
	it('should sort a list of ISO date strings', () => {
		const strings = ["2020-01-05", "2020-01-01", "2015-01-01"];
		const sorted = sortISOStrings(strings);
		expect(sorted).toEqual(["2015-01-01", "2020-01-01", "2020-01-05"]);
	});
});
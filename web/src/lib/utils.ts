/** Takes a page path and resolves a route, returning undefined if no route matches. */
export function getBasePath(path: string): string {
	return `/${path.split("/").filter(a => a)[0]}`;
}

export enum Weekday {
	Monday = 1,
	Tuesday = 2,
	Wednesday = 3,
	Thursday = 4,
	Friday = 5,
	Saturday = 6,
	Sunday = 0
}

/**
 * Returns the date on the start of a week defined by firstDay.
 */
export function getStartOfWeek(date: Date, firstDay = Weekday.Monday): Date {
	const currentDay = new Date(date).getDay();
	let diff = currentDay - firstDay;
	if (diff < 0) {
		diff = 7 + diff;
	}
	return new Date(new Date(date).setDate(date.getDate() - diff));
}

/**
 * Returns a new date with `days` added to it. 
 * A new object is created which triggers view invalidation for Svelte.
 */
export function addDays(date: Date, days: number): Date {
	const updated = new Date(date);
	updated.setDate(date.getDate() + days);
	return updated;
}

/**
 * Returns a short version of a date's day.
 */
export function getDayAbbr(date: Date) {
	const shortDays = ['SUN', 'MON', 'TUE', 'WED', 'THU', 'FRI', 'SAT'];
	return shortDays[date.getDay()];
}

/**
 * Returns a short version of a date's day.
 */
export function getDay(date: Date) {
	const shortDays = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
	return shortDays[date.getDay()];
}

/**
 * Returns the YYYY-MM-DD portion of a date's ISO date.
 */
export function toISOString(date: Date): string {
	return date.toISOString().split('T')[0];
}

/**
 * Sorts a list of ISO date strings
 */
export function sortISOStrings(strings: string[]) {
	return strings.sort(function (a, b) {
		return new Date(b) > new Date(a) ? -1
			: new Date(b) < new Date(a) ? 1
				: 0;
	});
}
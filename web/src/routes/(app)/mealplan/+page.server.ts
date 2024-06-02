import { getDays } from "$lib/client/day";
import { addDays, getStartOfWeek, toISOString } from "$lib/utils.js";

export async function load({ url }) {
	// If the current week has been overridden by the URL, use that instead of the current date.
	const week = new URLSearchParams(url.search).get('week');

	const today = new Date(Date.now());
	const from = week ? new Date(week) : getStartOfWeek(today);
	const to = addDays(from, 7);
	const isPresent = toISOString(from) === toISOString(getStartOfWeek(today));
	const days = await getDays(from, to);

	return {
		days,
		from,
		to,
		today,
		isPresent
	};
}
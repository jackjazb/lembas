<script lang="ts">
	import { getDays, groupByDate } from "$lib/client/day.js";
	import CardList from "$lib/components/CardList.svelte";
	import DayCard from "$lib/components/DayCard.svelte";
	import IconCalendar from "$lib/components/icons/IconCalendar.svelte";
	import { topBarProps } from "$lib/stores";
	import {
		addDays,
		getDayAbbr,
		getStartOfWeek,
		sortISOStrings,
		toISOString,
	} from "$lib/utils";
	export let data;

	$topBarProps = {
		title: "Meal Plan",
		actions: [],
	};

	// Links to the previous and next week.
	const weekLink = (date: Date) => {
		const params = new URLSearchParams({
			week: toISOString(date),
		});
		return `/mealplan?${params.toString()}`;
	};

	$: nextWeek = weekLink(addDays(data.from, 7));
	$: prevWeek = weekLink(addDays(data.from, -7));

	// The week's planned recipes grouped by date.
	$: daysByDate = groupByDate(data.days);

	// Dates to display in the top selector.
	$: weekDates = Array.from(new Array(7).keys()).map((i) =>
		addDays(data.from, i),
	);
</script>

<div class="join w-full justify-center h-max">
	{#each weekDates as date}
		<button
			class={`join-item py-2 btn btn-sm flex flex-col h-auto ${date.getDate() === data.today.getDate() && "font-bold"}`}
		>
			<div>{getDayAbbr(date)}</div>
			<div>{date.getDate()}</div>
		</button>
	{/each}
</div>

<div class="join w-full justify-center my-4">
	<a class="join-item btn h-auto" href={prevWeek}>«</a>

	{#if data.isPresent}
		<a
			class="join-item btn pointer-events-none brightness-50"
			href="/mealplan"><IconCalendar /></a
		>
	{:else}
		<a class="join-item btn" href="/mealplan"><IconCalendar /></a>
	{/if}
	<a class="join-item btn h-auto" href={nextWeek}>»</a>
</div>
<CardList>
	{#each sortISOStrings(Object.keys(daysByDate)) as key}
		<DayCard days={daysByDate[key]} date={new Date(key)} />
	{/each}
</CardList>

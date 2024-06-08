<script lang="ts">
	import { getDays, groupByDate } from "$lib/client/day.js";
	import CardList from "$lib/components/CardList.svelte";
	import DayCard from "$lib/components/DayCard.svelte";
	import IconCalendar from "$lib/components/icons/IconCalendar.svelte";
	import IconWand from "$lib/components/icons/IconWand.svelte";
	import { topBarProps } from "$lib/stores";
	import {
		addDays,
		getStartOfWeek,
		getWeekdayShort,
		sortISOStrings,
		toISOString,
	} from "$lib/utils";

	export let data;

	$topBarProps = {
		title: "Meal Plan",
		actions: [],
	};

	// Generates a link to a week's plan.
	const weekLink = (date: Date) => `/mealplan/${toISOString(date)}`;

	// Links to the subsequent and previous weeks.
	$: nextWeek = weekLink(addDays(data.from, 7));
	$: prevWeek = weekLink(addDays(data.from, -7));

	// This week's planned recipes grouped by date.
	$: daysByDate = groupByDate(data.days);

	// Dates to display in the top selector.
	$: weekDates = Array.from(new Array(7).keys()).map((i) =>
		addDays(data.from, i),
	);
</script>

<div class="join w-full justify-center h-max">
	{#each weekDates as date}
		<a
			class={`join-item py-2 btn btn-sm flex flex-col h-auto ${date.getDate() === data.today.getDate() && "font-bold"}`}
			href={`/mealplan/${toISOString(date)}/add`}
		>
			<div>{getWeekdayShort(date)}</div>
			<div>{date.getDate()}</div>
		</a>
	{/each}
</div>

<div class="join w-full justify-center my-4">
	<a class="join-item btn h-auto" href={prevWeek}>«</a>

	<a
		class={`join-item btn ${data.isPresent && "pointer-events-none brightness-75"}`}
		href="/mealplan"><IconCalendar /></a
	>
	<a class="join-item btn h-auto" href={nextWeek}>»</a>
</div>

<a
	class="btn btn-primary btn-square fixed right-4 bottom-20"
	href={`/mealplan/${toISOString(data.from)}/list`}><IconWand /></a
>

<CardList>
	{#each sortISOStrings(Object.keys(daysByDate)) as key}
		<DayCard days={daysByDate[key]} date={new Date(key)} />
	{/each}
</CardList>

<div class="mb-16"></div>

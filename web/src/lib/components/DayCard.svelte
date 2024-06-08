<script lang="ts">
	import { invalidateAll } from "$app/navigation";
	import { deleteDay, type Day } from "$lib/client/day";
	import IconDelete from "$lib/components/icons/IconDelete.svelte";
	import { ToastType, addToast } from "$lib/stores";
	import { getWeekday } from "$lib/utils";

	export let date: Date;
	export let days: Day[];

	const recipeLink = (id: string) => `/recipes/${id}`;

	const doDeleteDay = async (id: string) => {
		try {
			await deleteDay(id);
			invalidateAll();
		} catch (e) {
			console.log(e);
			addToast("Error: Failed to update meal plan.", ToastType.Error);
		}
	};
</script>

<!--
	@component
	Takes a date and a list of `Day` objects and renders their recipes on a card.
	
-->

<div class="w-full flex flex-col gap-2">
	<div class="divider"></div>
	<h2 class="card-title font-bold">{getWeekday(date)}</h2>
	{#each days as day}
		<div class="join w-full">
			<a
				class="join-item btn justify-start flex-1"
				href={recipeLink(day.recipe.recipe_id)}
			>
				{day.recipe.title}
			</a>

			<button
				class="btn join-item"
				on:click={() => doDeleteDay(day.day_id)}><IconDelete /></button
			>
		</div>
	{/each}
	<!-- <div class="card-actions justify-end">
			<a class="btn btn-primary" href={`recipes/${recipe.recipe_id}`}
				>View</a
			>
		</div> -->
</div>

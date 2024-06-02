<script lang="ts">
	import type { Day } from "$lib/client/day";
	import { getDay } from "$lib/utils";

	export let date: Date;
	export let days: Day[];
</script>

<!--
	@component
	Takes a date and a list of `Day` objects and renders their recipes on a card.
	
-->

<div class="card w-full shadow-xl">
	<div class="card-body">
		<h2 class="card-title font-bold">{getDay(date)}</h2>
		{#each days as day}
			{day.recipe.title}
		{/each}
		<!-- <div class="card-actions justify-end">
			<a class="btn btn-primary" href={`recipes/${recipe.recipe_id}`}
				>View</a
			>
		</div> -->
	</div>
</div>

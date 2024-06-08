<script lang="ts">
	import { goto } from "$app/navigation";
	import { createDay } from "$lib/client/day.js";
	import type { Recipe } from "$lib/client/recipe.js";
	import { ToastType, addToast, topBarProps } from "$lib/stores.js";
	import { formatDate } from "$lib/utils.js";

	export let data;

	const doCreateDay = async (recipe: Recipe) => {
		try {
			await createDay(recipe, data.date);
			history.back();
		} catch (e) {
			console.log(e);

			addToast("Error: Failed to update meal plan.", ToastType.Error);
		}
	};

	$topBarProps = {
		title: "Select Recipe",
		backEnabled: true,
		actions: [],
	};
</script>

<div>
	<h1 class="text-2xl font-bold">
		What do you want to cook on {formatDate(data.date)}?
	</h1>
	<div class="divider"></div>
	<div class="flex flex-col">
		{#each data.recipes as recipe}
			<button
				class="hover:cursor-pointer"
				on:click={() => doCreateDay(recipe)}
			>
				<h1>
					{recipe.title}
				</h1>
				<div class="divider"></div>
			</button>
		{/each}
	</div>
</div>

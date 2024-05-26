<script lang="ts">
	import RecipeCard from "$lib/components/RecipeCard.svelte";
	import IconAdd from "$lib/components/icons/IconAdd.svelte";
	import { topBarProps } from "$lib/stores.js";
	$topBarProps = {
		title: "My Recipes",
		actions: [
			{
				icon: IconAdd,
				action: "/recipes/add",
			},
		],
	};
	export let data;
</script>

<div class="flex gap-4 flex-col">
	{#each data.recipes as recipe}
		<RecipeCard {recipe} />
	{/each}
</div>

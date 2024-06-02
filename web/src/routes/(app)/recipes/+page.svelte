<script lang="ts">
	import CardList from "$lib/components/CardList.svelte";
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

<CardList>
	{#each data.recipes as recipe}
		<RecipeCard {recipe} />
	{/each}
</CardList>

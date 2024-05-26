<script lang="ts">
	import { goto } from "$app/navigation";
	import { formatIngredient } from "$lib/client/ingredient.js";
	import { deleteRecipe } from "$lib/client/recipe.js";
	import IconDelete from "$lib/components/icons/IconDelete.svelte";
	import { ToastType, addToast, topBarProps } from "$lib/stores.js";
	export let data;

	const doDeleteRecipe = async () => {
		try {
			await deleteRecipe(data.recipe.recipe_id);
			goto("/recipes");
		} catch (e) {
			console.log(e);

			addToast("Error: Failed to delete recipe.", ToastType.Error);
		}
	};

	$topBarProps = {
		title: data.recipe.title,
		backURL: "/recipes",
		actions: [
			{
				icon: IconDelete,
				action: doDeleteRecipe,
			},
		],
	};
</script>

<article class="prose">
	<h1>{data.recipe.title}</h1>
	<h2>Ingredients</h2>
	<ul>
		{#each data.recipe.ingredients as ingredient}
			<li>{formatIngredient(ingredient)}</li>
		{/each}
	</ul>
	<div class="divider" />
	<h2>Steps</h2>
	<ol>
		{#each data.recipe.steps as step}
			<li>{step}</li>
		{/each}
	</ol>
</article>

<script lang="ts">
	import { formatIngredient } from "$lib/client/ingredient";
	import { topBarProps } from "$lib/stores";

	export let data;
	$topBarProps = {
		title: "Shopping List",
		backEnabled: true,
		actions: [],
	};
</script>

{#each data.list as ingredient}
	<div>
		<h1>{formatIngredient(ingredient.ingredient)}</h1>
		<h2>
			Minimum purchase: {ingredient.units *
				ingredient.ingredient.purchase_unit}
		</h2>
		<div class="divider" />
	</div>
{/each}

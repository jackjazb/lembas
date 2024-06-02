<script lang="ts">
	import type { Recipe } from "$lib/client/recipe";

	export let recipe: Recipe;
</script>

<div class="card w-full bg-base-100 shadow-xl">
	<figure>
		<div class="bg-primary w-full h-20" />
	</figure>
	<div class="card-body">
		<h2 class="card-title font-bold">{recipe.title}</h2>
		<div class="card-actions justify-end">
			<a class="btn btn-primary" href={`recipes/${recipe.recipe_id}`}
				>View</a
			>
		</div>
	</div>
</div>

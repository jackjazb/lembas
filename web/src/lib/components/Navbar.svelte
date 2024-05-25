<script lang="ts">
	import { ROUTES, type Route } from "$lib/routes";

	export let active: Route | undefined;
</script>

<!--
	@component
	Renders a navbar with the the active route highlighted.
	If the active route is undefined, no routes are highlighted.
	
-->
<div class="btm-nav">
	{#each Object.values(ROUTES) as route}
		{#if active && route.url === active.url}
			<a href={route.url} class="active">
				<svelte:component this={route.icon} filled />
			</a>
		{:else}
			<a href={route.url}>
				<svelte:component this={route.icon} />
			</a>
		{/if}
	{/each}
</div>

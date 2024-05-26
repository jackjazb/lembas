<script lang="ts">
	import { page } from "$app/stores";
	import IconApple from "$lib/components/icons/IconApple.svelte";
	import IconClipboard from "$lib/components/icons/IconClipboard.svelte";
	import IconPackage from "$lib/components/icons/IconPackage.svelte";
	import Toast from "$lib/components/Toast.svelte";
	import { toast } from "$lib/stores";
	import { getBasePath } from "$lib/utils";
	import { type ComponentType } from "svelte";
	import { fade } from "svelte/transition";

	$: activeBasePath = getBasePath($page.url.pathname);
	const routeIcons: Record<string, ComponentType> = {
		"/recipes": IconApple,
		"/mealplan": IconClipboard,
		"/ingredients": IconPackage,
	};
</script>

<!--
	@component
	Renders a navbar with the the active route highlighted.
	If the active route is undefined, no routes are highlighted.
	
-->
<div class="mb-12 m4 toast" transition:fade>
	{#if $toast}
		<Toast toast={$toast} />
	{/if}
</div>

<div class="btm-nav">
	{#each Object.keys(routeIcons) as path}
		{#if path === activeBasePath}
			<a href={path} class="active">
				<svelte:component this={routeIcons[path]} filled />
			</a>
		{:else}
			<a href={path}>
				<svelte:component this={routeIcons[path]} />
			</a>
		{/if}
	{/each}
</div>

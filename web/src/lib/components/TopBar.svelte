<script lang="ts">
	import { browser } from "$app/environment";
	import IconBack from "$lib/components/icons/IconBack.svelte";
	import type { ComponentType } from "svelte";

	/** Defines a single top bar action*/
	interface TopBarAction {
		icon: ComponentType;
		action: string | (() => void);
	}

	export let title: string;
	export let backEnabled: boolean = false;
	export let actions: TopBarAction[] = [];
</script>

<!--
	@component
	Renders a top bar with a title.
	
	Props:
	- `title`: The text displayed in the left of the bar.render.
	- `backURL`: (Optional) Renders a link styled as a back button with this URL if present.
	- `actions`: Renders a link or button for each on the right, depending on whether action is a string or function.
-->

<div class="navbar bg-base-200">
	{#if backEnabled}
		<button class="btn btn-square" on:click={() => history.back()}
			><IconBack /></button
		>
	{/if}
	<div class="flex-1">
		<h1 class="text-xl font-bold m-2">{title}</h1>
	</div>
	{#if actions}
		{#each actions as action}
			{#if typeof action.action === "string"}
				<a class="btn btn-square" href={action.action}>
					<svelte:component this={action.icon} />
				</a>
			{:else}
				<button class="btn btn-square" on:click={action.action}>
					<svelte:component this={action.icon} />
				</button>
			{/if}
		{/each}
	{/if}
</div>

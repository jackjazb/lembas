<script lang="ts">
	import IconError from "$lib/components/icons/IconError.svelte";
	import { ToastType, removeToast, type Toast } from "$lib/stores";

	export let toast: Toast;
</script>

{#if toast.type === ToastType.Error}
	<div role="alert" class="alert alert-error my-4">
		<IconError />
		<h1>{toast.message}</h1>
	</div>
{:else if toast.type === ToastType.Success}
	<div role="alert" class="alert alert-success my-4">
		<IconError />
		<h1>{toast.message}</h1>
	</div>
{:else}
	<div role="alert" class="alert alert-info my-4">
		<IconError />
		<h1>{toast.message}</h1>
	</div>
{/if}

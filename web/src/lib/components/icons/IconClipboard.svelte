<script lang="ts">
	import TwoVariantIcon from "$lib/components/icons/TwoVariantIcon.svelte";
	import MdiClipboard from "virtual:icons/mdi/clipboard-text";
	import MdiClipboardOutline from "virtual:icons/mdi/clipboard-text-outline";

	export let filled = false;
</script>

<TwoVariantIcon {filled}>
	<MdiClipboard slot="filled" />
	<MdiClipboardOutline slot="outlined" />
</TwoVariantIcon>

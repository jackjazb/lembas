<script lang="ts">
	import TwoVariantIcon from "$lib/components/icons/TwoVariantIcon.svelte";
	import MdiPackageOpen from "virtual:icons/mdi/package-variant";
	import MdiPackageClosed from "virtual:icons/mdi/package-variant-closed";

	export let filled = false;
</script>

<TwoVariantIcon {filled}>
	<MdiPackageOpen slot="filled" />
	<MdiPackageClosed slot="outlined" />
</TwoVariantIcon>

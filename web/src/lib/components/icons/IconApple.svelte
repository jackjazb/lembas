<script lang="ts">
	import TwoVariantIcon from "$lib/components/icons/TwoVariantIcon.svelte";
	import MdiFoodApple from "virtual:icons/mdi/food-apple";
	import MdiFoodAppleOutline from "virtual:icons/mdi/food-apple-outline";

	export let filled = false;
</script>

<TwoVariantIcon {filled}>
	<MdiFoodApple slot="filled" />
	<MdiFoodAppleOutline slot="outlined" />
</TwoVariantIcon>

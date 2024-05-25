<script lang="ts">
	export let filled = false;
</script>

<!--
@component
A generic wrapper for a filled and outlined variant of a single icon.

Usage:
	```tsx
	<TwoVariantIcon {filled}>
		<IconFilled slot="filled" />
		<IconOutlined slot="outlined" />
	</TwoVariantIcon>
	```
-->

{#if filled}
	<slot name="filled" />
{:else}
	<slot name="outlined" />
{/if}

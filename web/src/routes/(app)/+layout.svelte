<script>
	import Navbar from "$lib/components/Navbar.svelte";
	import TopBar from "$lib/components/TopBar.svelte";
	import { topBarProps } from "$lib/stores";
</script>

<TopBar {...$topBarProps} />
<div class="m-4 pb-16">
	<slot />
</div>

<Navbar />

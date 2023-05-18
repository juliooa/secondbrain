<script>
	import '@skeletonlabs/skeleton/themes/theme-gold-nouveau.css';
	import '@skeletonlabs/skeleton/styles/all.css';
	import '../app.postcss';
	import SideMenu from '$lib/components/SideMenu.svelte';
</script>

<SideMenu>
	<slot />
</SideMenu>

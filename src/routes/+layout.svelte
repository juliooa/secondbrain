<script>
	import '@skeletonlabs/skeleton/themes/theme-gold-nouveau.css';
	import '@skeletonlabs/skeleton/styles/all.css';
	import '../app.postcss';
	import SideMenu from '$lib/components/SideMenu.svelte';
	import { Modal } from '@skeletonlabs/skeleton';
</script>

<Modal />
<SideMenu>
	<slot />
</SideMenu>

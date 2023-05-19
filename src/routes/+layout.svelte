<script lang="ts">
	import '@skeletonlabs/skeleton/themes/theme-gold-nouveau.css';
	import '@skeletonlabs/skeleton/styles/all.css';
	import '../app.postcss';
	import SideMenu from '$lib/components/SideMenu.svelte';
	import { Modal } from '@skeletonlabs/skeleton';
	import { Store } from 'tauri-plugin-store-api';
</script>

<Modal />
<SideMenu>
	<slot />
</SideMenu>

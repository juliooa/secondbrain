<script lang="ts">
	import '@skeletonlabs/skeleton/themes/theme-gold-nouveau.css';
	import '@skeletonlabs/skeleton/styles/all.css';
	import '../app.postcss';
	import SideMenu from '$lib/components/SideMenu.svelte';
	import { Modal, Toast } from '@skeletonlabs/skeleton';
</script>

<Toast />
<Modal />

<div class="bg-surface-50-900-token rounded-none grid grid-cols-[auto_1fr] w-full">
	<SideMenu>
		<slot />
	</SideMenu>
</div>

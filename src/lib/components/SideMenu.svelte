<script lang="ts">
	import { AppRail, AppRailTile, Avatar } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';
	import { derived } from 'svelte/store';
	import 'iconify-icon';
	import logo from '$lib/assets/secondbrain_icon.png';
	const selected = derived(page, ($page) => $page.url.pathname);
</script>

<div class="bg-surface-50-900-token rounded-none h-full grid grid-cols-[auto_1fr] w-full">
	<AppRail {selected}>
		<AppRailTile hover=""><Avatar src={logo} width="w-12" rounded="rounded-full" /></AppRailTile>
		<AppRailTile label="Q/A" href="/qa"
			><iconify-icon width="28" icon="mdi:robot-happy-outline" /></AppRailTile
		>
		<AppRailTile label="Chat" href="/chat"
			><iconify-icon width="28" icon="ion:chatbubbles-outline" /></AppRailTile
		>
		<AppRailTile label="Documents" href="/own_documents"
			><iconify-icon width="28" icon="ion:documents-outline" /></AppRailTile
		>
		<svelte:fragment slot="trail">
			<AppRailTile label="Config" href="/config"
				><iconify-icon width="28" icon="ion:settings" /></AppRailTile
			>
		</svelte:fragment>
	</AppRail>
	<div class="w-full">
		<slot />
	</div>
</div>

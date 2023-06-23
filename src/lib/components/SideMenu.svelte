<script lang="ts">
	import { AppRail, AppRailTile, Avatar } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';
	import { derived } from 'svelte/store';
	import Breathing from './Breathing.svelte';
	import IconModel from './icons/IconModel.svelte';
	import IconSettings from './icons/IconSettings.svelte';
	import IconDocuments from './icons/IconDocuments.svelte';
	import IconChat from './icons/IconChat.svelte';
	import IconQA from './icons/IconQA.svelte';
	import IconModelSettings from './icons/IconModelSettings.svelte';
	const selected = derived(page, ($page) => $page.url.pathname);
</script>

<AppRail {selected} height="h-screen">
	<AppRailTile hover="" href="/">
		<Breathing /></AppRailTile
	>
	<AppRailTile label="Q/A" href="/qa">
		<IconQA />
	</AppRailTile>
	<AppRailTile label="Chat" href="/chat">
		<IconChat />
	</AppRailTile>
	<AppRailTile label="Documents" href="/own_documents">
		<IconDocuments />
	</AppRailTile>
	<svelte:fragment slot="trail">
		<AppRailTile label="Parameters" href="/parameters">
			<IconModelSettings />
		</AppRailTile>
		<AppRailTile label="Models" href="/models">
			<IconModel />
		</AppRailTile>
	</svelte:fragment>
</AppRail>
<div class="w-full flex flex-col max-h-screen h-screen">
	<div class="flex-grow">
		<slot />
	</div>
</div>

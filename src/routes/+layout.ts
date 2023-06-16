import type { ActiveLanguageModel } from '$lib/types';
import type { LayoutLoad } from './$types';
import { invoke } from '@tauri-apps/api/tauri';

export const prerender = true;
export const ssr = false;

export const load = (async ({ depends }) => {
	let activeModel = await invoke<ActiveLanguageModel>('get_active_model');
	depends('app:activeModelChanged');
	return {
		activeModel: activeModel
	};
}) satisfies LayoutLoad;

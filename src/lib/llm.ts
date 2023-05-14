import { invoke } from '@tauri-apps/api';

async function infere(text: string) {
	return await invoke('infere', { text: text });
}

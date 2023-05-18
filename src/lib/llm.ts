import { invoke } from '@tauri-apps/api';

export async function infere(text: string): Promise<string> {
	return await invoke('infere', { text: text });
}

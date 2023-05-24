import { invoke } from '@tauri-apps/api';

export async function chat(message: string): Promise<string> {
	return await invoke('chat', { message: message });
}

export async function ask(message: string): Promise<string> {
	return await invoke('ask', { message: message });
}

import { invoke } from '@tauri-apps/api';

export async function chat(message: string): Promise<string> {
	return await invoke('chat', { message: message });
}

export async function ask(message: string): Promise<string> {
	return await invoke('ask', { message: message });
}

export async function cancelInference(): Promise<boolean> {
	return await invoke('cancel_inference');
}

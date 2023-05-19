import { Store } from 'tauri-plugin-store-api';

export async function getModelName(): Promise<string | null> {
	let store: Store = new Store('store.bin');
	return await store.get<string>('current_model_name');
}

export async function getCurrentModelId(): Promise<number | null> {
	let store: Store = new Store('store.bin');
	return await store.get<number>('current_model_id');
}

export async function getCurrentModelName(): Promise<string | null> {
	let store: Store = new Store('store.bin');
	return await store.get<string>('current_model_name');
}

import { Store } from 'tauri-plugin-store-api';

export class LocalStore {
	private store: Store = new Store('store.bin');
	private static instance: LocalStore | null = null;

	private constructor() {
		// Private constructor to prevent direct instantiation
	}

	public static async getInstance(): Promise<LocalStore> {
		if (!LocalStore.instance) {
			LocalStore.instance = new LocalStore();
			await LocalStore.instance.initialize();
		}
		return LocalStore.instance;
	}

	private async initialize(): Promise<void> {
		await this.store.load();
	}

	public async getModelName() {
		return await this.store.get<string>('current_model_name');
	}
}

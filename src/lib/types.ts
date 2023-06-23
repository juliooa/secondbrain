export type Message = {
	id: number;
	text: string;
	role: MessageRole;
};

export enum MessageRole {
	HUMAN = 'human',
	AI = 'AI'
}

export type LanguageModel = {
	filename: string;
	current: boolean;
	downloaded: boolean;
	has_info: boolean;
	isDownloading: boolean;
	info: LanguageModelInfo;
};

export type LanguageModelInfo = {
	name: string;
	arquitecture: string;
	url: string;
	image: string;
	prompt_template: string;
	size: string;
};

export type CommandResponseLanguagesModels = {
	models: LanguageModel[];
};

export interface NewTokenPayload {
	message: String;
}

export interface TextBlock {
	isCodeBlock: boolean;
	text: string;
	language?: string;
}

export type ActiveLanguageModel = {
	filename: string;
	name: string;
	path: string;
};

export interface DownloadProgress {
	model_filename: string;
	progress: number;
	total: number;
	percentage: number;
}

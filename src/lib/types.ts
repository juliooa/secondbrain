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
	name: string;
	arquitecture: string;
	url: string;
	image: string;
	downloaded: boolean;
	current: boolean;
	prompt_base: string;
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

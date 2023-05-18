export type Message = {
	id: number;
	text: string;
	role: string;
};

export type LanguageModel = {
	id: number;
	name: string;
	url: string;
	filename: string;
	image: string;
	downloaded: boolean;
	default: boolean;
};

export type CommandResponseLanguagesModels = {
	models: LanguageModel[];
};

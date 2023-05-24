import type { TextBlock } from './types';

export function parseText(text: string): TextBlock[] {
	const regex = /```([\w-]+)?\s*([\s\S]+?)\s*```/g;
	const blocks: TextBlock[] = [];
	let lastIndex = 0;
	let match;

	while ((match = regex.exec(text))) {
		const [fullMatch, language, code] = match;
		const preMatch = text.slice(lastIndex, match.index);
		if (preMatch) {
			blocks.push({ isCodeBlock: false, text: preMatch });
		}
		blocks.push({ isCodeBlock: true, text: code, language });
		lastIndex = match.index + fullMatch.length;
	}

	const lastBlock = text.slice(lastIndex);
	if (lastBlock) {
		blocks.push({ isCodeBlock: false, text: lastBlock });
	}

	return blocks;
}

export function generateRandomId(): number {
	return Math.floor(Math.random() * 100000);
}

import { toastStore } from '@skeletonlabs/skeleton';

export function error(message: string) {
	triggerToast(message, 'variant-filled-error');
}

export function success(message: string) {
	triggerToast(message, 'variant-filled-success');
}

function triggerToast(message: string, background: string) {
	toastStore.trigger({
		message: message,
		autohide: false,
		background: background,
		classes: 'text-white'
	});
}

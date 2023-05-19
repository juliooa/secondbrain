<script>
	import { Avatar } from '@skeletonlabs/skeleton';
	import logo from '$lib/assets/secondbrain_icon.png';
	export let isBreathing = true;
</script>

<div class="icon {isBreathing ? 'breathing' : ''}">
	<Avatar src={logo} width="w-12" rounded="rounded-full" />
</div>

<style>
	.breathing {
		animation: breathingAnimation 3s infinite;
	}

	@keyframes breathingAnimation {
		0% {
			transform: scale(1);
		}
		50% {
			transform: scale(1.1);
		}
		100% {
			transform: scale(1);
		}
	}
</style>

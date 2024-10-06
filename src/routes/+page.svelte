<script lang="ts">
	import { onMount } from 'svelte';

	let isRecording = false;
	onMount(() => {
		// Add keyboard event listeners
		window.addEventListener('keydown', handleKeyDown);
		window.addEventListener('keyup', handleKeyUp);
	});
	

	function handleKeyDown(event: KeyboardEvent) {
    if (event.code === 'Space' && !event.repeat && !isRecording) {
      event.preventDefault(); // Prevent page scrolling
      startRecording();
    }
  }

	function handleKeyUp(event: KeyboardEvent) {
    if (event.code === 'Space' && isRecording) {
      stopRecording();
    }
  }

	function startRecording() {
		isRecording = true;
	}
	function stopRecording() {
		isRecording = false;
	}
</script>

<div class="container">
	<h1>Talk to me</h1>
	<button
		on:mousedown={startRecording}
		on:mouseup={stopRecording}
		on:mouseleave={stopRecording}
		on:touchstart={startRecording}
		on:touchend={stopRecording}
		class:recording={isRecording}
	>
		{isRecording ? "Recording..." : "Hold to Record"}
	</button>
</div>

<style>
	.container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-between;
		height: 90vh;
	}
	button.recording {
		background-color: #f44336;
	}
	button {
		padding: 20px 20px;
		width: 240px;
		font-size: 1.5em;
		user-select: none;
	}

	@media (prefers-color-scheme: dark) {
		:root {
			background-color: #333;
			color: #fff;
		}
	}
</style>

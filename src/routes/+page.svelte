<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import AudioVisualiser from "../components/AudioVisualiser.svelte";

  let isRecording = false;
  let isMicOn = false;
  let transcript = "";

  onMount(() => {
    // Add keyboard event listeners
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
  });

  function handleKeyDown(event: KeyboardEvent) {
    if (event.code === "Space" && !event.repeat && !isRecording) {
      event.preventDefault(); // Prevent page scrolling
      startRecording();
    }
  }

  function handleKeyUp(event: KeyboardEvent) {
    if (event.code === "Space" && isRecording) {
      stopRecording();
    }
  }

  function startRecording() {
    invoke("start_microphone").then(() => {
      isRecording = true;
    });
  }
  function stopRecording() {
    invoke("stop_microphone").then(() => {
      isRecording = false;
    });
  }

  function startMic() {
    invoke("start_microphone").then(() => {
      isMicOn = true;
    });
  }

  listen<string>("transcript", (event) => {
    transcript = event.payload;
  });

	
	let data = Array.from({ length: 4000 }, () => Math.random());
  listen<number[]>("audio_data", (event) => {
		data = event.payload;
  });

	// Random set of 4000 values between 1 and 0
</script>

<div class="container">
  <h1>Talk to me, boy</h1>
  <div class="transcript">{transcript}</div>

	<AudioVisualiser data={data} />

  <div>
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
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    height: 90vh;
  }
  .transcript {
    font-size: 1.5em;
    margin: 20px 0;
    white-space: pre-wrap;
    text-align: center;
  }
  button.recording {
    background-color: #f44336;
  }
  button {
    padding: 20px 20px;
    width: 240px;
    font-size: 1.5em;
    user-select: none;
    border: none;
    border-radius: 5px;
    cursor: pointer;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      background-color: #333;
      color: #fff;
    }
  }
</style>

<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";

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
		invoke('ping', 'allo').then((response) => {
			console.log(response);
		});
    invoke("start_recording").then(() => {
      isRecording = true;
    });
  }
  function stopRecording() {
    invoke("stop_recording").then(() => {
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
</script>

<div class="container">
  <h1>Talk to me, boy</h1>
  <div class="transcript">{transcript}</div>

  <div>
    <button on:click={startMic}>Start mic</button>

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

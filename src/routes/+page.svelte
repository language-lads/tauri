<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import AudioVisualiser from "../components/AudioVisualiser.svelte";

  let data: number[];
  $: data = [];
  listen<number[]>("audio_data", (event) => {
    data = event.payload;
  });

  let toggleAudioSessionPromise: Promise<void>;
  let audioSessionActive = false;
  async function toggleAudioSession() {
    if (audioSessionActive) {
      await new Promise((resolve) => setTimeout(resolve, 500));
      await invoke("stop_audio_session").catch(console.error);
    } else {
      await new Promise((resolve) => setTimeout(resolve, 500));
      await invoke("start_audio_session").catch(console.error);
    }
    audioSessionActive = !audioSessionActive;
  }

  function handleClick() {
    toggleAudioSessionPromise = toggleAudioSession();
  }

  function getPermissions() {
    invoke("get_permissions").then((response) => {
      console.log(response);
    });
  }
</script>

<div class="container">
  <h1>Talk to me, boy</h1>

  <AudioVisualiser {data} />

  <div class="button-container">
    <button on:click={getPermissions}> Get permissions </button>
    {#await toggleAudioSessionPromise}
      <button class:recording={audioSessionActive} disabled>
        {audioSessionActive ? "Stopping..." : "Starting..."}
      </button>
    {:then}
      <button on:click={handleClick} class:recording={audioSessionActive}>
        {audioSessionActive ? "I'm listening!" : "Start conversation"}
      </button>
    {/await}
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
  .button-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 20px;
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

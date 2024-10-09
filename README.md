# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## TODO

- Setup tauri-plugin-microphone so that it properly asks for permissions and stuff at the right time
- Write appium tests to make sure that the microphone is working

## config

```
RUSTFLAGS="--cfg=break_audio_input_device" npm run tauri dev
RUSTFLAGS="--cfg=break_default_input_device_config" npm run tauri dev
```

Realtime API stuff
- microphone thread -> *Get audio from the microphone* pushes to audio input channel
- websocket sender thread -> *Send audio to OpenAI* input from audio input channel, send to WS
- websocket listener thread -> *Receive audio from OpenAI* input from WS, send to audio output channel
- audio output thread -> *Play audio back to the user* plays audio

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

- microphone thread -> _Get audio from the microphone_ pushes to audio input channel
- websocket sender thread -> _Send audio to OpenAI_ input from audio input channel, send to WS
- websocket listener thread -> _Receive audio from OpenAI_ input from WS, send to audio output channel
- audio output thread -> _Play audio back to the user_ plays audio

## Helpful commands

View the logs from the connected android device / emulator:

```bash
adb shell run-as com.lisalanguagelads.app logcat | grep --line-buffered lisa_lib
```

TODO: setup the getting of microphone permissions because it's annoying

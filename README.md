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

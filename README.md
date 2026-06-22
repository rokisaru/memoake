# memoake

A friction-free, lightweight dropdown scratchpad designed for developers who want to dump their thoughts without leaving the home position.

https://github.com/user-attachments/assets/2406979f-8b73-48a3-a8e2-6ea224e2afa7

## Overview

`memoake` is a keyboard-driven, overlay-style scratchpad inspired by **Yakuake**. With a single global shortcut, it drops down instantly from the top of your screen and vanishes just as quickly when you're done. Notes are appended to daily Markdown files, so folders such as OneDrive can sync them without a database export step.

## Features

* **Instant Dropdown Toggle:** Zero startup delay. The application runs as a lightweight background daemon, allowing you to show or hide the overlay window in milliseconds via a global hotkey.
* **Markdown Storage:** Each confirmed memo is appended to `YYYY-MM-DD.md` with a timestamp heading.
* **Configurable Folder:** The save directory defaults to `Documents/memoake` and can be changed from the app.
* **Minimal Footprint:** Powered by **Tauri**, **Rust**, and **Svelte 5**. Unlike bloated Electron-based alternatives, it keeps memory and CPU consumption to an absolute minimum, making it ideal for a permanent resident in your OS environment.

## Background

As developers, we constantly need a place to jot down transient snippets, error logs, or quick thoughts. Creating temporary `.txt` or `.md` files manually introduces unnecessary cognitive friction: *"Where should I save it?"*, *"What should I name it?"*, and *"Do I need to track this in Git?"*

`memoake` was born out of the frustration with this overhead. You don't need a heavy, cloud-synced knowledge base just to scratch something down. You just need a blistering-fast buffer that captures your thoughts in 0.5 seconds, writes them to plain Markdown, and stays completely out of your way.

## Windows Notes

Windows 10 / 11 x64 is the primary release target. The installer uses the Tauri WebView2 download bootstrapper, which keeps the installer small while still handling machines that do not already have the runtime.

Unsigned local builds may show a Windows Defender SmartScreen warning. Choose **More info** and then **Run anyway** only when you built the binary yourself or trust the release source.

## Resource Targets

The app is designed to preserve Tauri's lightweight advantage:

* Hidden/background: target 50 MB or less, investigate above 80 MB.
* Window visible: target 80 MB or less, investigate above 120 MB.
* Idle CPU: target 0.1% or less, investigate above 1%.

See `docs/markdown_storage_spec.md` for implementation and verification notes.

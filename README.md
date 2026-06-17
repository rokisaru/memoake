# memoake

A friction-free, lightweight dropdown scratchpad designed for developers who want to dump their thoughts without leaving the home position.

https://github.com/user-attachments/assets/2406979f-8b73-48a3-a8e2-6ea224e2afa7

## Overview

`memoake` is a keyboard-driven, overlay-style scratchpad inspired by **Yakuake**. With a single global shortcut, it drops down instantly from the top of your screen and vanishes just as quickly when you're done. Built with a minimalist philosophy, it eliminates the overhead of managing files or context switching during deep work.

## Features

* **Instant Dropdown Toggle:** Zero startup delay. The application runs as a lightweight background daemon, allowing you to show or hide the overlay window in milliseconds via a global hotkey.
* **100% Keyboard-Driven:** No mouse required. From creating and searching notes to deleting them, every operation is mapped to efficient, distraction-free keybindings.
* **Minimal Footprint:** Powered by **Tauri**, **Rust**, and **Svelte 5**. Unlike bloated Electron-based alternatives, it keeps memory and CPU consumption to an absolute minimum, making it ideal for a permanent resident in your OS environment.

## Background

As developers, we constantly need a place to jot down transient snippets, error logs, or quick thoughts. Creating temporary `.txt` or `.md` files manually introduces unnecessary cognitive friction: *"Where should I save it?"*, *"What should I name it?"*, and *"Do I need to track this in Git?"*

`memoake` was born out of the frustration with this overhead. You don't need a heavy, cloud-synced knowledge base just to scratch something down. You just need a volatile, blistering-fast buffer that captures your thoughts in 0.5 seconds and stays completely out of your way.

# memoake Markdown Storage Spec

## 1. Purpose

Replace SQLite persistence with append-only daily Markdown files named `YYYY-MM-DD.md`.
This lets OneDrive and other tools index memoake notes directly as plain UTF-8 text.

## 2. Scope

Included:

- Save-directory configuration.
- Daily Markdown file creation and append.
- Timestamped memo entries.
- JSON config stored under Tauri `app_config_dir()`.

Not included in phase 1:

- Historical memo list/search UI.
- Past memo editing.
- Cloud sync logic.
- SQLite migration.

## 3. Behavior

On memo confirmation:

1. Load the configured save directory.
2. Build the current day's Markdown filename from `filename_format`.
3. Create the directory and file if needed.
4. Append a timestamped entry.
5. Clear the input and hide the window only after a successful write.

New files start with:

```markdown
---
date: 2026-06-23
created_by: memoake
---

# 2026-06-23
```

Entries are appended as:

```markdown
## 14:32

Memo body.
Multiple lines are preserved.
```

Files are written as UTF-8 without BOM and LF line endings.

## 4. Configuration

| Key | Default | Notes |
| --- | --- | --- |
| `save_directory` | `Documents/memoake` | Absolute path. Uses Tauri `document_dir()` by default. |
| `filename_format` | `%Y-%m-%d` | `strftime` format. `.md` is added automatically. |
| `timestamp_heading_level` | `2` | Supports `2`, `3`, or `4`. |
| `timestamp_format` | `%H:%M` | `strftime` format. |

## 5. Windows Support

Windows 10 / 11 x64 is the primary target.

- Use `PathBuf` and `join`; do not construct filesystem paths by concatenating separators.
- Default save path is `%USERPROFILE%\Documents\memoake` via `document_dir()`.
- OneDrive paths and Japanese usernames must work as normal Unicode paths.
- Filename output is rejected if it contains Windows-forbidden characters: `< > : " / \ | ? *`.
- Global shortcut support remains based on Tauri's global shortcut plugin.
- Windows installers use WebView2 `downloadBootstrapper`.
- Unsigned builds can trigger SmartScreen; README documents the expected warning.

## 6. Memory And Resource Targets

| State | Target | Investigate above |
| --- | --- | --- |
| Hidden tray/background window | 50 MB or less | 80 MB |
| Window visible | 80 MB or less | 120 MB |
| Idle CPU | 0.1% or less | 1% |

Implementation constraints:

- No Markdown preview, rich text editor, or heavy UI framework.
- SQLite crates are removed in phase 1.
- Rust additions are limited to `chrono` and Tauri plugins needed by the UI.
- File I/O happens only on confirmation, using synchronous `std::fs`.
- Release builds optimize for binary size.

## 7. Verification

Acceptance checks:

- First launch creates the default config.
- Saving a memo creates the daily Markdown file with front matter and heading.
- A second memo on the same day appends to the same file.
- Changing the save directory affects the next save.
- Deleted save directories are recreated.
- Failed writes keep the textarea content.
- Japanese text is preserved as UTF-8.
- Invalid filename formats such as `%H:%M` return an error on Windows-prohibited `:`.

Before release, record:

- Task Manager memory usage while hidden and visible.
- Windows installer size.
- Cold and warm startup time.

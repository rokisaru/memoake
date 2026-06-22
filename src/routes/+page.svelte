<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";

  interface AppConfig {
    save_directory: string;
    filename_format: string;
    timestamp_heading_level: number;
    timestamp_format: string;
  }

  let body = $state("");
  let config = $state<AppConfig>({
    save_directory: "",
    filename_format: "%Y-%m-%d",
    timestamp_heading_level: 2,
    timestamp_format: "%H:%M",
  });
  let showSettings = $state(false);
  let isSaving = $state(false);
  let statusMessage = $state("");
  let errorMessage = $state("");

  function autoFocus(element: HTMLTextAreaElement) {
    element.focus();
  }

  onMount(async () => {
    try {
      config = await invoke<AppConfig>("get_config");
    } catch (err) {
      errorMessage = String(err);
      showSettings = true;
    }
  });

  async function saveMemo() {
    const memo = body.trim();
    if (!memo || isSaving) return;

    isSaving = true;
    errorMessage = "";
    statusMessage = "";

    try {
      const savedPath = await invoke<string>("save_memo", { body: memo });
      body = "";
      statusMessage = savedPath;
      await invoke("hide_main_window");
    } catch (err) {
      errorMessage = String(err);
    } finally {
      isSaving = false;
    }
  }

  async function saveConfig() {
    errorMessage = "";
    statusMessage = "";

    try {
      config = await invoke<AppConfig>("update_config", {
        newConfig: {
          ...config,
          timestamp_heading_level: Number(config.timestamp_heading_level),
        },
      });
      showSettings = false;
      statusMessage = "Settings saved.";
    } catch (err) {
      errorMessage = String(err);
    }
  }

  $effect(() => {
    invoke("set_compact_mode", { compact: !showSettings }).catch((err) => {
      console.error("failed to resize window", err);
    });
  });

  async function pickFolder() {
    const selected = await open({ directory: true, multiple: false });
    if (typeof selected === "string") {
      config.save_directory = selected;
    }
  }

  async function handleKeyDown(event: KeyboardEvent) {
    if ((event.ctrlKey || event.metaKey) && event.key === "Enter") {
      event.preventDefault();
      await saveMemo();
      return;
    }

    if (event.key === "Escape") {
      if (showSettings) {
        showSettings = false;
        return;
      }
      await invoke("hide_main_window");
    }
  }
</script>

<svelte:window on:keydown={handleKeyDown} />

<main class="container">
  <header class="topbar">
    <div class="brand">memoake</div>
    <div class="actions">
      <button class="ghost" type="button" onclick={() => (showSettings = !showSettings)}>
        Settings
      </button>
      <button class="primary" type="button" disabled={isSaving || !body.trim()} onclick={saveMemo}>
        {isSaving ? "Saving" : "Save"}
      </button>
    </div>
  </header>

  {#if showSettings}
    <section class="settings">
      <label>
        <span>Save directory</span>
        <div class="path-row">
          <input bind:value={config.save_directory} />
          <button class="ghost" type="button" onclick={pickFolder}>Browse</button>
        </div>
      </label>

      <div class="settings-grid">
        <label>
          <span>Filename format</span>
          <input bind:value={config.filename_format} />
        </label>

        <label>
          <span>Timestamp format</span>
          <input bind:value={config.timestamp_format} />
        </label>

        <label>
          <span>Heading level</span>
          <select bind:value={config.timestamp_heading_level}>
            <option value={2}>##</option>
            <option value={3}>###</option>
            <option value={4}>####</option>
          </select>
        </label>
      </div>

      <div class="settings-actions">
        <button class="ghost" type="button" onclick={() => (showSettings = false)}>
          Cancel
        </button>
        <button class="primary" type="button" onclick={saveConfig}>Apply</button>
      </div>
    </section>
  {/if}

  {#if errorMessage}
    <div class="notice error">{errorMessage}</div>
  {:else if statusMessage && showSettings}
    <div class="notice success">{statusMessage}</div>
  {/if}

  <textarea
    class="editor"
    bind:value={body}
    placeholder="Input your awesome memos..."
    use:autoFocus
  ></textarea>
</main>

<style>
  :global(*),
  :global(*::before),
  :global(*::after) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    font-family:
      Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI",
      sans-serif;
    background: #1f2330;
    color: #f0f3f8;
    overflow: hidden;
  }

  button,
  input,
  select,
  textarea {
    font: inherit;
  }

  button {
    min-height: 34px;
    border: 1px solid transparent;
    border-radius: 6px;
    padding: 0 12px;
    color: #f0f3f8;
    cursor: pointer;
  }

  button:disabled {
    cursor: default;
    opacity: 0.45;
  }

  .container {
    display: flex;
    flex-direction: column;
    gap: 12px;
    height: 100vh;
    padding: 16px;
  }

  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    min-height: 38px;
  }

  .brand {
    font-size: 0.95rem;
    font-weight: 700;
    color: #b9f27c;
  }

  .actions,
  .settings-actions,
  .path-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .ghost {
    background: #2a3041;
    border-color: #3a4258;
  }

  .ghost:hover {
    background: #333b50;
  }

  .primary {
    background: #3c6df0;
    border-color: #4b79f4;
  }

  .primary:hover:enabled {
    background: #4a7af8;
  }

  .settings {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 12px;
    border: 1px solid #394258;
    border-radius: 8px;
    background: #252b3a;
  }

  label {
    display: flex;
    flex-direction: column;
    gap: 6px;
    color: #c6cde0;
    font-size: 0.82rem;
  }

  input,
  select {
    width: 100%;
    min-height: 34px;
    border: 1px solid #3a4258;
    border-radius: 6px;
    background: #171b25;
    color: #f0f3f8;
    padding: 0 10px;
    outline: none;
  }

  input:focus,
  select:focus,
  textarea:focus {
    border-color: #71a7ff;
  }

  .path-row input {
    min-width: 0;
  }

  .settings-grid {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr) 120px;
    gap: 10px;
  }

  .settings-actions {
    justify-content: flex-end;
  }

  .notice {
    border-radius: 6px;
    padding: 8px 10px;
    font-size: 0.85rem;
  }

  .error {
    background: #4a2630;
    color: #ffd8df;
    border: 1px solid #7f4251;
  }

  .success {
    background: #203b31;
    color: #c8f7df;
    border: 1px solid #36664f;
  }

  .editor {
    width: 100%;
    min-height: 0;
    flex: 1;
    resize: none;
    border: 1px solid #394258;
    border-radius: 8px;
    background: #10141d;
    color: #f0f3f8;
    padding: 16px;
    outline: none;
    line-height: 1.6;
    font-family:
      "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
    font-size: 1rem;
  }

  @media (max-width: 640px) {
    .topbar,
    .path-row {
      align-items: stretch;
      flex-direction: column;
    }

    .actions {
      width: 100%;
    }

    .actions button,
    .path-row button {
      flex: 1;
    }

    .settings-grid {
      grid-template-columns: 1fr;
    }
  }
</style>

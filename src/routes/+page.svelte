<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  interface Memo {
    id: number;
    content: string;
    created_at: string;
    updated_at: string;
  }

  let memos = $state<Memo[]>([]);
  let activeId = $state<number | null>(null);
  let currentMemo = $derived(memos.find((p) => p.id === activeId));
  let isConfirmingDelete = $state(false);

  const getTitle = (content: string) => {
    return content.trim().split("\n")[0] || "No title";
  };

  async function handleKeyDown(event: KeyboardEvent) {
    const key = event.key.toLocaleLowerCase();
    switch (key) {
      case "escape":
        if (currentMemo === undefined || activeId === null) return;
        activeId = null;
        isConfirmingDelete = false;
        event.preventDefault();
        return;
      case "d":
        if (!event.ctrlKey) return;
        event.preventDefault();
        isConfirmingDelete = true;
        return;
      // 削除の確認（Yes）
      case "y":
        if (!isConfirmingDelete || currentMemo === undefined) return;
        const idToDelete = currentMemo.id;
        try {
          await invoke("delete_memo", { id: idToDelete });
          memos = memos.filter((m) => m.id !== idToDelete);
          activeId = null;
          isConfirmingDelete = false;
        } catch (err) {
          console.log("Failed to delete a memo", err);
        }
        return;
      // 削除の確認（No）
      case "n":
        if (!isConfirmingDelete || currentMemo === undefined) return;
        event.preventDefault();
        isConfirmingDelete = false;
        return;
      case "c":
        const newMemo: Memo = await invoke("create_memo", { content: "" });
        memos = [...memos, newMemo];
        activeId = newMemo.id;
        event.preventDefault();
        return;
        break;
    }

    // 数字を指定してメモを選択して開く
    if (key >= "1" && key <= "9" && activeId === null) {
      const index = parseInt(event.key) - 1;
      if (memos[index]) {
        activeId = memos[index].id;
        event.preventDefault();
      }
      return;
    }

    if (event.ctrlKey && event.key === "z") {
      event.preventDefault();
      document.execCommand("undo", false);
      return
    } else if (event.ctrlKey && event.key === "Z") {
      event.preventDefault();
      document.execCommand("redo", false);
      return
    }
  }

  function autoFocus(element: HTMLTextAreaElement) {
    element.focus();
  }

  let timeoutId: number | undefined;
  $effect(() => {
    console.log("effect!!!");
    if (activeId !== null && currentMemo) {
      const c = currentMemo.content;
      if (timeoutId) clearTimeout(timeoutId);

      timeoutId = setTimeout(async () => {
        try {
          const updated_memo: Memo = await invoke("update_memo", {
            id: currentMemo.id,
            content: c,
          });
          currentMemo.updated_at = updated_memo.updated_at;
        } catch (err) {
          console.error("failed to restore data", err);
        }
      }, 1000);
    }
  });

  $effect.pre(() => {
    async function getAllMemos() {
      memos = await invoke("get_all_memo");
    }
    getAllMemos();
  });
</script>

<svelte:window on:keydown={handleKeyDown} />

<main class="container">
  {#if activeId === null}
    <div class="header">
      <div class="logo">memoake</div>
      <div>
        <span>[C] Create new memo</span>
        <span style="color: #45475a; margin: 0 10px;">|</span>
        <span>[1-9] Open memo</span>
      </div>
    </div>

    <div class="grid">
      {#each memos as project, index}
        <button class="tile" onclick={() => (activeId = project.id)}>
          <div class="tile-header">
            <span class="badge">[{index + 1}]</span>
            <span class="tile-footer">{project.updated_at}</span>
          </div>
          <div class="tile-title">{getTitle(project.content)}</div>
          <div class="tile-preview">
            {project.content.split("\n").slice(1).join(" ") || "Empty memo"}
          </div>
        </button>
      {/each}
    </div>
  {:else if currentMemo}
    <div class="editor-container">
      <div class="editor-header" class:danger={isConfirmingDelete}>
        {#if isConfirmingDelete}
          <span class="alert-text"
            >⚠️ Are you sure you want to delete this memo? [Y] Yes / [N] No</span
          >
        {:else}
          <span>[Ctrl+D] Delete</span>
          <span style="color: #45475a; margin: 0 10px;">|</span>
          <span>[Esc] Return to list</span>
        {/if}
      </div>
      <textarea
        class="editor"
        class:blur={isConfirmingDelete}
        bind:value={currentMemo.content}
        placeholder="Input your awesome memos..."
        use:autoFocus
      ></textarea>
    </div>
  {/if}
</main>

<style>
  :global(*),
  :global(*::before),
  :global(*::after) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    font-family: sans-serif;
    background: #1e1e2e;
    color: #cdd6f4;
    overflow: hidden;
  }

  .container {
    padding: 20px;
    height: 100vh;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    border-bottom: 1px solid #313244;
    padding-bottom: 10px;
  }

  .logo {
    font-weight: bold;
    color: #cba6f7;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 15px;
  }

  .tile {
    background: #252538;
    border: 1px solid #313244;
    border-radius: 8px;
    padding: 15px;
    text-align: left;
    cursor: pointer;
    color: inherit;
    display: flex;
    flex-direction: column;
    height: 140px;
    transition: background 0.1s ease;
  }

  .tile:hover {
    background: #313244;
    border-color: #cba6f7;
  }

  .tile-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    margin-bottom: 8px;
  }

  .badge {
    background: #cba6f7;
    color: #11111b;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: bold;
  }

  .tile-title {
    font-weight: bold;
    font-size: 1.1rem;
    color: #f5c2e7;
    margin-bottom: 8px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tile-preview {
    font-size: 0.9rem;
    color: #bac2de;
    flex-grow: 1;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .tile-footer {
    font-size: 0.75rem;
    color: #585b70;
  }

  .editor-container {
    display: flex;
    flex-direction: column;
    flex-grow: 1;
  }

  .editor-header {
    font-size: 0.85rem;
    color: #a6adc8;
    margin-bottom: 10px;
    padding: 6px 10px;
    border-radius: 4px;
    transition:
      background 0.15s ease,
      color 0.15s ease;
  }

  .editor-header.danger {
    background: #f38ba8; /* CatppuccinのRed */
    color: #11111b; /* 暗い背景色にして文字をクッキリ */
    font-weight: bold;
  }

  .alert-text {
    animation: blink 1s infinite alternate; /* 文字を少しだけ明滅させて緊張感を出す */
  }

  /* 確認中、テキストエリアをうっすら暗くしてフォーカスがヘッダーにあることを示す */
  .editor.blur {
    opacity: 0.5;
    border-color: #f38ba8;
  }

  @keyframes blink {
    0% {
      opacity: 0.8;
    }
    100% {
      opacity: 1;
    }
  }

  .editor {
    width: 100%;
    box-sizing: border-box;
    flex-grow: 1;
    background: #11111b;
    color: #cdd6f4;
    border: 1px solid #313244;
    border-radius: 8px;
    padding: 15px;
    font-size: 1.05rem;
    line-height: 1.6;
    resize: none;
    outline: none;
    font-family: monospace;
  }
</style>

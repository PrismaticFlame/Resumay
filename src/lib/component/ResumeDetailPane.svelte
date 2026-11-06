<script lang="ts">
    import { timeFormat } from "$lib/time-format";
    import { invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";

    let { resume, ondelete } = $props();
    // let confirming = $state(false);
    // let editing = $state(false);

    let message = $state("");
    let previewUrl = $state("");

    let confirming = $state(false);

    async function showResume(id: number) {
      message = "";
      try {
        const bytes = await invoke<number[]>("fetch_resume", { 
          id 
        });
        const blob = new Blob([new Uint8Array(bytes)], { type: "application/pdf" });
        if (previewUrl) URL.revokeObjectURL(previewUrl);
        previewUrl = URL.createObjectURL(blob);
      } catch (err) {
        message = String(err);
      }
    }

    onDestroy(() => { if (previewUrl) URL.revokeObjectURL(previewUrl); });
    onMount(() => showResume(resume.id));
</script>

<div class="pane">
  
  <div class="info">
    <h2>{resume.label}</h2>
    <p class="role">Number of uses in Applications: {resume.uses}</p>
    <h2>Date Created</h2>
    <p class="role">{timeFormat(resume.created_at)}</p>

    <h3>Applications Using This Resume:</h3>

    <div class="danger">
      {#if resume.uses > 0}
        <span class="locked">
          <b>Used by {resume.uses} application{resume.uses === 1 ? "" : "s"}, so it can't be deleted.</b>
        </span>
      {:else if confirming}
        <span>Delete this Resume?</span>
        <button class="del" onclick={ondelete}>Yes, delete</button>
        <button onclick={() => confirming = false}>Cancel</button>
      {:else}
        <button class="del-ghost" onclick={() => confirming = true}>Delete</button>
      {/if}
    </div>

  </div>

  <div class="viewer">
    <h2>Resume</h2>
    <!-- <button class="view" onclick={() => showResume(resume.id)}>View PDF</button> -->
    {#if message}<p class="error">{message}</p>{/if}
    {#if previewUrl}
        <iframe title="Resume preview" src={previewUrl}></iframe>
    {/if}
  </div>

</div>

<style>
    .pane { display: grid; grid-template-columns: 500px 1fr; gap: 24px;
        height: 100%; padding: 24px; box-sizing: border-box; }
    .info { min-height: 0; overflow-y: auto; }
    .viewer { min-height: 0; }
    iframe { width: 100%; height: 100%; border: 0; }
    h2 { margin: 10px 0 2px; font-size: 22px; font-weight: 600; }
    .role { margin: 0 0 20px; }
    .display { min-height: 44px; padding: 0 18px; border: none; border-radius: 8px;
    background: var(--accent); color: #fff; font-weight: 600; cursor: pointer; }
    .view {
    min-height: 44px;
    padding: 0 18px;
    border: 1px solid var(--accent);
    border-radius: 8px;
    background: var(--accent-weak);
    color: var(--accent);
    font-weight: 600;
    cursor: pointer;
    margin-bottom: 12px;
    }
    .view:hover { filter: brightness(1.08); }
    .view:active { filter: brightness(0.95); }
    .view:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
    .view:disabled { opacity: 0.5; cursor: default; }
    h3 { margin: 20px 0 8px; font-size: 12px; font-weight: 600; color: var(--muted);
         text-transform: uppercase; letter-spacing: .06em; }
    .mono { font-family: 'IBM Plex Mono', monospace; font-size: 13px; }
    .danger { display: flex; align-items: center; gap: 8px; margin-top: 28px;
          padding-top: 16px; padding-left: 16px; border-top: 1px solid var(--line); }
    .danger button { min-height: 44px; padding: 0 14px; border-radius: 8px;
                    border: 1px solid #D1D5DB; background: #fff; cursor: pointer; }
    .del { background: #B91C1C !important; border-color: #B91C1C !important; color: #fff; }
    .del-ghost { color: #B91C1C; }
    
</style>
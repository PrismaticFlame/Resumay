<script lang="ts">
    import { timeFormat } from "$lib/time-format";
    import { invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";
    import type { ApplicationSummary } from "$lib/types";

    import StatusPill from "./StatusPill.svelte";

    let { resume, ondelete } = $props();
    // let confirming = $state(false);
    // let editing = $state(false);

    let apps = $state<ApplicationSummary[]>([]);
    onMount(async () => {
      apps = await invoke<ApplicationSummary[]>("fetch_application_summary", { id: resume.id});
    });

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

    <h2>Applications Using This Resume:</h2>

    <ul class="apps">
      {#each apps as a (a.id)}
        <li>
          <span class="who"><b>{a.company}</b><small>{a.position}</small></span>
          <StatusPill status={a.status} />
          <span class="mono">{a.apply_date}</span>
        </li>
      {:else}
        <li class="empty">Not used by any application yet.</li>
    {/each}
    </ul>
    
    {#if resume.uses === 0}
      <div class="danger">
        {#if confirming}
          <span>Delete this Resume?</span>
          <button class="del" onclick={ondelete}>Yes, delete</button>
          <button onclick={() => confirming = false}>Cancel</button>
        {:else}
          <button class="del-ghost" onclick={() => confirming = true}>Delete</button>
        {/if}
      </div>
    {/if}

  </div>

  <div class="viewer">
    <h2>Resume</h2>
    <!-- <button class="view" onclick={() => showResume(resume.id)}>View PDF</button> -->
    {#if message}<p class="error">{message}</p>{/if}
    {#if previewUrl}
        <iframe title="Resume preview" src="{previewUrl}#toolbar=0&navpanes=0"></iframe>
    {/if}
  </div>

</div>

<style>
  .pane { display: grid; grid-template-columns: 500px 1fr; gap: 24px;
      height: 100%; padding: 24px; box-sizing: border-box; }
  .info { min-height: 0; overflow-y: auto; }
  .viewer { min-height: 0; overflow: hidden; }
  iframe { width: 100%; height: calc(100% + 56px); margin-top: -83px; border: none; }
  h2 { margin: 10px 0 2px; font-size: 22px; font-weight: 600; }
  .role { margin: 0 0 20px; }
  h3 { margin: 20px 0 8px; font-size: 12px; font-weight: 600; color: var(--muted);
        text-transform: uppercase; letter-spacing: .06em; }
  .danger { display: flex; align-items: center; gap: 8px; margin-top: 28px;
        padding-top: 16px; padding-left: 16px; border-top: 1px solid var(--line); }
  .danger button { min-height: 44px; padding: 0 14px; border-radius: 8px;
                  border: 1px solid #D1D5DB; background: #fff; cursor: pointer; }
  .del { background: #B91C1C !important; border-color: #B91C1C !important; color: #fff; }
  .del-ghost { color: #B91C1C; }
  .mono { font-family: 'IBM Plex Mono', monospace; font-size: 13px; }
  .apps { list-style: none; margin: 0; padding: 0; }
  .apps li { display: grid; grid-template-columns: 1fr auto auto; gap: 12px;
            align-items: center; padding: 10px 0; border-bottom: 1px solid var(--line); }
  .who { display: flex; flex-direction: column; min-width: 0; }
  .who small { color: var(--muted); }
  .empty { display: block; color: var(--muted); }
</style>
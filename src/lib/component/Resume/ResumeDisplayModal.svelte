<script lang="ts">
    import type { Resume } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";

    interface Props {
        resumeId: number,
        label? : string | null;
        onclose: () => void;
    }
    
    let { resumeId, label, onclose }: Props = $props();

    let message = $state("");
    let previewUrl = $state("");
    let dialog = $state<HTMLDialogElement>();

    $effect(() => {
        dialog?.showModal();
    });

    // const url = URL.createObjectURL(new Blob([buf], { type: "application/pdf" }));

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

    onDestroy(() => { if (previewUrl) URL.revokeObjectURL(previewUrl) });
    onMount(() => showResume(resumeId));
</script>

<dialog
    bind:this={dialog}
    aria-label="Resume"
    onclose={onclose}
    onclick={(e) => e.target === e.currentTarget && e.currentTarget.close()}
>
    <div class="panel">
        <h2>{`Resume Name: ${label || "No Name"}`}</h2>
        {#if message}<p class="error">{message}</p>{/if}
        {#if previewUrl}
            <iframe title="Resume preview" src="{previewUrl}#toolbar=0&navpanes=0&zoom=page-fit&view=Fit"></iframe>
        {/if}
    </div>
</dialog>

<style>
    dialog { 
        padding: 0;
        width: min(900px, 92vw);
        height: 92vh;
        background: var(--surface);
        color: var(--ink);
        border: 1px solid var(--line);
        border-radius: 12px;
        box-shadow: 0 20px 50px rgba(0, 0, 0, .25);
        overflow: hidden;
    }
    dialog::backdrop {
        background: rgba(0, 0, 0, .4);
        backdrop-filter: blur(6px);
    }
    .panel { 
        padding: 24px;
        height: 100%;
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }
    iframe {
        flex: 1;
        width: 100%;
        border: none;
        border-radius: 8px;
    }

</style>
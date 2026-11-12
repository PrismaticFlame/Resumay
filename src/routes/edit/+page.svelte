<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onDestroy } from 'svelte';
    
    import Rail from "$lib/component/Rail.svelte";

    let compileError = $state("");
    let compiling = $state(false);
    let latexSource = $state("");
    let pdfBlobUrl = $state("");

    let message = $state("");

    async function compileLatex() {
        if (compiling) return;
        compiling = true;
        compileError = "";
        try {
            const bytes = await invoke<number[]>("compile_resume", {
                tex: latexSource
            });
            const blob = new Blob([new Uint8Array(bytes)], { type: "application/pdf" });
            if (pdfBlobUrl) URL.revokeObjectURL(pdfBlobUrl);
            pdfBlobUrl = URL.createObjectURL(blob);
        } catch (err) {
            compileError = String(err);
        } finally {
            compiling = false;
        }
    }

    onDestroy(() => { 
        if (pdfBlobUrl) URL.revokeObjectURL(pdfBlobUrl); 
    });
</script>

<div class="shell">
    <Rail />
    <main>
        <header>
            <div>
                <h1>Editor</h1>
            </div>
        </header>

        <div class="panes">
            <div class="left">
                <div class="pane-head">
                    <span>Source</span>
                    <button class="primary" onclick={compileLatex} disabled={compiling}>
                        {compiling ? "Compiling..." : "Compile Latex"}
                    </button>
                </div>
                <textarea bind:value={latexSource} spellcheck="false"></textarea>
            </div>

            <div class="right">
                <div class="pane-head">
                    <span>Preview</span>
                    <button class="primary">
                        {"Save PDF"}
                    </button>
                </div>
                <div class="viewer">
                    {#if compileError}<pre class="error">{compileError}</pre>{/if}
                    {#if pdfBlobUrl}
                        <iframe title="Resume preview" src="{pdfBlobUrl}#toolbar=0&navpanes=0"></iframe>
                    {:else if !compileError}
                        <small>Compile to see the preview!</small>
                    {/if}
                </div>
            </div>
        </div>
        
    </main>
</div>

<style>
    .shell { display: grid; grid-template-columns: 224px 1fr; height: 100vh; }
    main { display: flex; flex-direction: column; min-height: 0; min-width: 0; }
    header { display: flex; align-items: center; gap: 16px; padding: 22px 28px 14px; }
    header div { flex: 1; }
    h1 { margin: 0; font-size: 24px; font-weight: 600; }
    .panes {
        flex: 1;
        min-height: 0;
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 16px;
        padding: 0 28px 22px;
    }
    .left, .right {
        display: flex;
        flex-direction: column;
        min-height: 0;
        min-width: 0;
        background: var(--surface);
        border: 1px solid var(--line);
        border-radius: 10px;
        overflow: hidden;
    }
    .pane-head {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 10px 14px;
        border-bottom: 1px solid var(--line);
        background: #F9FAFB;
        font-size: 12px;
        font-weight: 600;
        color: var(--muted);
        text-transform: uppercase;
        letter-spacing: .05em;
    }
    textarea {
        flex: 1;
        resize: none;
        border: none;
        outline: none;
        padding: 16px 18px;
        font-family: 'IBM Plex Mono', monospace;
        font-size: 13px;
        line-height: 1.6;
        tab-size: 4;
        color: var(--ink);
    }
    .viewer { 
        flex: 1; 
        min-height: 0; 
        background: #E5E7EB;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    iframe { width: 100%; height: 100%; border: 0; }
    button:disabled {
        opacity: 0.6;
        cursor: wait;
    }
    .error {
        align-self: stretch;
        margin: 0;
        padding: 14px 16px;
        background: #FEF2F2;
        color: #991B1B;
        font-family: 'IBM Plex Mono', monospace;
        font-size: 12px;
        white-space: pre-wrap;
        overflow: auto;
    }
</style>
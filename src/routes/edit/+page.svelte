<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from 'svelte';
    import { save } from "@tauri-apps/plugin-dialog"

    let compileError = $state("");
    let compiling = $state(false);
    let latexSource = $state("");
    let pdfBlobUrl = $state("");
    let lastPdf: number[] = [];

    let message = $state("");

    let loaded = $state(false);

    async function compileLatex() {
        if (compiling) return;
        compiling = true;
        compileError = "";
        try {
            const bytes = await invoke<number[]>("compile_resume", {
                tex: latexSource
            });
            lastPdf = bytes;
            const blob = new Blob([new Uint8Array(bytes)], { type: "application/pdf" });
            if (pdfBlobUrl) URL.revokeObjectURL(pdfBlobUrl);
            pdfBlobUrl = URL.createObjectURL(blob);
        } catch (err) {
            compileError = String(err);
        } finally {
            compiling = false;
        }
    }

    async function saveTex() {
        const path = await save ({
            defaultPath: "resume.tex",
            filters: [{ name: "LaTeX", extensions: ["tex"] }],
        });
        if (!path) return;
        await invoke("write_text", { path, contents: latexSource });
    }

    async function savePdf() {
        const path = await save ({
            defaultPath: "resume.pdf",
            filters: [{ name: "resume", extensions: ["pdf"] }],
        });
        if (!path) return;
        await invoke("write_bytes", { path, contents: lastPdf });
    }

    onMount(async () => {
        latexSource = await invoke<string>("load_draft");
        loaded = true;
    });

    $effect(() => {
        const tex = latexSource;
        if (!loaded) return;
        const t = setTimeout(() => invoke("save_draft", { tex }), 800);
        return () => clearTimeout(t);
    })

    onDestroy(() => { 
        if (pdfBlobUrl) URL.revokeObjectURL(pdfBlobUrl); 
    });
</script>


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
                <div class="pane-actions">
                    <button class="primary" onclick={compileLatex} disabled={compiling}>
                        {compiling ? "Compiling..." : "Compile Latex"}
                    </button>
                    <button class="ghost" onclick={saveTex}>Save .tex</button>
                </div>
            </div>
            <textarea 
            bind:value={latexSource} 
            spellcheck="false" 
            placeholder={"\\documentclass{article}\n\\begin{document}\nHello, World!\n\\end{document}"}>
            </textarea>
        </div>

        <div class="right">
            <div class="pane-head">
                <span>Preview</span>
                <button disabled={!pdfBlobUrl} class="primary" onclick={savePdf}>
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


<style>
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
    .pane-actions {
        display: flex;
        gap: 8px;
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
        background: var(--surface-2);
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
        background: var(--surface)
    }
    .viewer { 
        flex: 1; 
        min-height: 0; 
        background: var(--line);
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
        background: var(--danger-bg);
        color: var(--danger);
        font-family: 'IBM Plex Mono', monospace;
        font-size: 12px;
        white-space: pre-wrap;
        overflow: auto;
    }
</style>
<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let { onclose, onsaved, app = null } = $props();

    let dragging = $state(false);
    let resumeFile = $state<File | null>(null);
    let resumeId = $state<number | null>(app?.resume ?? null);
    let previewUrl = $state("");
    let fileInput = $state<HTMLInputElement | null>(null);
    let saving = $state(false);

    let error = $state("");

    let dialog = $state<HTMLDialogElement | null>(null);

    async function submit(event: Event) {
        error = "";
        event.preventDefault();
        try {
            if (!resumeFile) {
                error = "Choose a PDF first";
                return;
            }
            saving = true;
            const id = await uploadResume(resumeFile);
            onsaved(id);
            onclose();
        } catch (err) {
            error = String(err);
        } finally {
            saving = false;
        }
    }

    function clearFile() {
        error = "";
        resumeFile = null;
        if (fileInput) fileInput.value = "";
    }

    async function uploadResume(file: File): Promise<number> {
        if (!file.name.toLowerCase().endsWith(".pdf"))
            throw new Error("Only PDF files are supported");
        if (file.size > 5 * 1024 * 1024) {
            throw new Error("PDF too large (5 MB max)");
        }
        const pdf = Array.from(new Uint8Array(await file.arrayBuffer()));
        return await invoke<number>("add_resume", { label: file.name, pdf });
    }

    $effect(() => {
        const file = resumeFile;
        const id = resumeId;
        let url = "";
        let cancelled = false;

        if (file) {
            url = URL.createObjectURL(file);
            previewUrl = url;
        } else if (id !== null) {
            try {
                (async () => {
                    const bytes = await invoke<number[]>("fetch_resumes", {
                        id,
                    });
                    if (cancelled) return;
                    const blob = new Blob([new Uint8Array(bytes)], {
                        type: "application/pdf",
                    });
                    url = URL.createObjectURL(blob);
                    previewUrl = url;
                })();
            } catch (err) {
                if (!cancelled) error = String(err);
            }
        } else {
            previewUrl = "";
        }

        return () => {
            cancelled = true;
            if (url) URL.revokeObjectURL(url);
        };
    });

    onMount(() => dialog?.showModal());
</script>

<dialog
    bind:this={dialog}
    aria-labelledby="new-title"
    {onclose}
    onclick={(e) => e.target === e.currentTarget && e.currentTarget.close()}
>
    <div class="panel">
        <h2 id="new-title">New Resume</h2>
        <form onsubmit={submit}>
            <label
                class="drop"
                class:over={dragging}
                ondragover={(e) => {
                    e.preventDefault();
                    dragging = true;
                }}
                ondragleave={() => (dragging = false)}
                ondrop={(e) => {
                    e.preventDefault();
                    dragging = false;
                    resumeFile = e.dataTransfer?.files[0] ?? null;
                    resumeId = null;
                }}
            >
                <input
                    type="file"
                    bind:this={fileInput}
                    accept=".pdf"
                    hidden
                    onchange={(e) => {
                        resumeFile = e.currentTarget.files?.[0] ?? null;
                        resumeId = null;
                    }}
                />
                {#if resumeFile}
                    <b>{resumeFile.name}</b>
                {:else}
                    <b>Drop your resume here</b>
                    <small>or click to browse · PDF</small>
                {/if}
            </label>

            <div class="preview">
                {#if previewUrl}
                    <iframe
                        title="Selected resume"
                        src="{previewUrl}#pagemode=none&zoom=page-fit"
                    ></iframe>
                {:else}
                    <p class="empty">
                        Choose or upload a resume to preview it here
                    </p>
                {/if}
            </div>

            <div class="actions">
                {#if error}<p class="error" role="alert">{error}</p>{/if}
                <button type="button" onclick={onclose}>Cancel</button>
                <button
                    type="submit"
                    class="primary"
                    disabled={!resumeFile || saving}>Save</button
                >
            </div>
        </form>
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
    form {
        flex: 1;
        min-height: 0;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }
    h2 {
        margin: 0 0 16px;
        font-size: 20px;
        font-weight: 600;
    }
    .actions,
    .error {
        grid-column: 1 / -1;
    }
    input {
        min-height: 44px;
        padding: 8px 12px;
        border: 1px solid var(--line);
        border-radius: 8px;
    }
    .actions {
        display: flex;
        justify-content: flex-end;
        gap: 10px;
        margin-top: 8px;
    }
    .actions button {
        min-height: 44px;
        padding: 0 18px;
        border-radius: 8px;
        border: 1px solid var(--btn-border);
        background: var(--btn-bg);
        cursor: pointer;
    }
    .actions .primary {
        background: var(--accent);
        border-color: var(--accent);
        color: var(--on-accent);
        font-weight: 600;
    }
    .error {
        margin: 0 auto 0 0;
        color: var(--danger);
        font-size: 13px;
    }
    .drop {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 6px;
        padding: 24px;
        text-align: center;
        border: 2px dashed var(--line);
        border-radius: 12px;
        background: var(--bg);
        cursor: pointer;
        padding: 14px 16px;
    }
    .drop.over {
        border-color: var(--accent);
        background: var(--accent-weak);
    }
    .drop small {
        color: var(--muted);
    }
    .preview {
        position: relative;
        flex: 1;
        min-height: 240px;
        border: 1px solid var(--line);
        border-radius: 8px;
        overflow: hidden;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    .preview iframe {
        position: absolute;
        inset: 0;
        width: 100%;
        height: 100%;
        border: 0;
    }
    .empty {
        color: var(--muted);
        font-size: 13px;
        text-align: center;
        padding: 0 16px;
    }
</style>

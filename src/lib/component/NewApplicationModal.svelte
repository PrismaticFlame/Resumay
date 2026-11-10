<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { STATUS } from "$lib/status";
    import { onMount } from "svelte";
    import type { Resume } from "$lib/types";
    import { timeFormat } from "$lib/time-format";
    import Page from "../../routes/+page.svelte";

    let { onclose, onsaved, app = null } = $props();

    let dragging = $state(false);
    let resumeFile = $state<File | null>(null);
    let resumeId = $state<number | null>(app?.resume ?? null);
    let previewUrl = $state("");
    let fileInput = $state<HTMLInputElement | null>(null);

    let resumes = $state<Resume[]>([]);

    let company = $state(app?.company ?? "");
    let position = $state(app?.position ?? "");
    let website = $state(app?.website ?? "");
    let jobDescription = $state(app?.job_description ?? "");
    let username = $state(app?.username ?? "");
    let password = $state("");
    let applyDate = $state(app?.apply_date ?? new Date().toLocaleDateString("en-CA"));
    let status = $state(app?.status ?? "applied");
    let notes = $state(app?.notes ?? "");
    let refer = $state(app?.refer ?? "");
    let error = $state("");
  

    async function submit(event: Event) {
        event.preventDefault();
        try {

          let resume = resumeId;
          if (resumeFile) resume = await uploadResume(resumeFile);

          if (app) {
            await invoke("update_application", {
              id: app.id, company, position, website, jobDescription, username, applyDate, status, notes, refer, password, resume
            });
            onsaved(app.id)
          } else {
            const id = await invoke<number>("add_application", {
                company, position, website, jobDescription, username, applyDate, status, notes, refer, password, resume
            }); 
            onsaved(id);
          }
        } catch(err) {
            error = String(err);
        }
    }

    async function uploadResume(file: File): Promise<number>{
      if (!file.name.toLowerCase().endsWith(".pdf")) throw new Error("Only PDF files are supported");
      if (file.size > 5 * 1024 * 1024) {
        throw new Error("PDF too large (5 MB max)");
      }
      const pdf = Array.from(new Uint8Array(await file.arrayBuffer()));
      return await invoke<number>("add_resume", { label: file.name, pdf });
    }

    function clearFile() {
      resumeFile = null;
      if (fileInput) fileInput.value = "";
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
          const bytes = await invoke<number[]>("fetch_resume", {
            id
          });
          if (cancelled) return;
          const blob = new Blob([new Uint8Array(bytes)], { type: "application/pdf" });
          if (previewUrl) URL.revokeObjectURL(previewUrl);
          previewUrl = URL.createObjectURL(blob);
        })();
        } catch (err) {
          if (!cancelled) error = String(error);
        }        
      } else {
        previewUrl = "";
      }

      return () => {
        cancelled = true;
        if (url) URL.revokeObjectURL(url);
      };
    });

    onMount(
      async () => { resumes = await invoke<Resume[]>("list_resumes"); 
    });
</script>

<div class="overlay">
    <div class="dialog" role="dialog" aria-modal="true" aria-labelledby="new-title">
        <h2 id="new-title">{app ? "Edit application" : "New application"}</h2>

        <form onsubmit={submit}>
          <div class="left">
            <!-- this causes the select and the drag-n-drop to affect each other so that one doesn't override the other -->
            <select bind:value={resumeId} onchange={clearFile}> 
                <option value={null}>No resume</option>
                {#each resumes as r (r.id)}
                  <option value={r.id}>{r.label} · {timeFormat(r.created_at)}</option>
                {/each}
              </select>

            <label class="drop" class:over={dragging}
              ondragover={(e) => { e.preventDefault(); dragging = true; }}
              ondragleave={() => dragging = false}
              ondrop={(e) => {
                e.preventDefault(); dragging = false;
                resumeFile = e.dataTransfer?.files[0] ?? null;
                resumeId = null;
              }}>
              <input type="file" bind:this={fileInput} accept=".pdf" hidden
              onchange={(e) => {
                resumeFile = e.currentTarget.files?.[0] ?? null; 
                resumeId = null;
              }} />
              {#if resumeFile}
                <b>{resumeFile.name}</b>
              {:else}
                <b>Drop your resume here</b>
                <small>or click to browse · PDF</small>
              {/if}
            </label>

            <div class="preview">
              {#if previewUrl}
                <iframe title="Selected resume" src="{previewUrl}#pagemode=none&zoom=page-fit"></iframe>
              {:else}
                <p class="empty">Choose or upload a resume to preview it here</p>
              {/if}
            </div>

          </div>

          <div class="right">
            <input placeholder="Company *" bind:value={company} required />
            <input placeholder="Position *" bind:value={position} required />
            <input placeholder="Website URL" bind:value={website} />
            <textarea placeholder="Job Description" bind:value={jobDescription}></textarea>
            <input type="date" bind:value={applyDate} />

            <select bind:value={status}>
                {#each Object.entries(STATUS) as [key, s]}
                <option value={key}>{s.label}</option>
                {/each}
            </select>

            <input placeholder="Referral" bind:value={refer} />
            <input placeholder="Username" bind:value={username} />
            <input type="password" bind:value={password} 
                placeholder={app ? "New password (blank = keep current)" : "Password"}/>
            <textarea placeholder="Notes" bind:value={notes}></textarea>
          </div>

          {#if error}<p class="error">{error}</p>{/if}

          <div class="actions">
              <button type="button" onclick={onclose}>Cancel</button>
              <button type="submit" class="primary">{app ? "Save changes" : "Save application"}</button>
          </div>
        </form>
    </div>
</div>

<style>
  .overlay { position: fixed; inset: 0; display: flex; align-items: center;
    justify-content: center; background: rgba(17, 24, 39, .38);
    backdrop-filter: blur(6px); }
  .dialog { width: min(980px, 92vw); max-height: 90vh; overflow: auto; padding: 24px;
    border-radius: 16px; background: rgba(255, 255, 255, .9);
    backdrop-filter: blur(20px); box-shadow: 0 24px 64px rgba(17, 24, 39, .28); }
  h2 { margin: 0 0 16px; font-size: 20px; font-weight: 600; }
  form { display: grid; grid-template-columns: 1fr 1fr; gap: 24px; }
  .right { display: flex; flex-direction: column; gap: 10px;}
  .actions, .error { grid-column: 1 / -1; }
  input, select, textarea { min-height: 44px; padding: 8px 12px;
    border: 1px solid #D1D5DB; border-radius: 8px; background: #fff; }
  .actions { display: flex; justify-content: flex-end; gap: 10px; margin-top: 8px; }
  .actions button { min-height: 44px; padding: 0 18px; border-radius: 8px;
    border: 1px solid #D1D5DB; background: #fff; cursor: pointer; }
  .actions .primary { background: var(--accent); border-color: var(--accent);
    color: #fff; font-weight: 600; }
  .error { margin: 0; color: #B91C1C; font-size: 13px; }
  .drop { display: flex; flex-direction: column; align-items: center;
  justify-content: center; gap: 6px; padding: 24px; text-align: center;
  border: 2px dashed #9CA3AF; border-radius: 12px; background: #fff; cursor: pointer; }
  .drop.over { border-color: var(--accent); background: var(--accent-weak); }
  .drop small { color: var(--muted); }
  .left { display: flex; flex-direction: column; gap: 12px; min-height: 0; }
  .preview { flex: 1; min-height: 300px; border: 1px solid var(--line); border-radius: 8px;
            overflow: hidden; display: flex; align-items: center; justify-content: center; }
  .preview iframe { width: 100%; height: 100%; border: 0; }
  .empty { color: var(--muted); font-size: 13px; text-align: center; padding: 0 16px; }
</style>
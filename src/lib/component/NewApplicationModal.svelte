<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { STATUS } from "$lib/status";

    let { onclose, onsaved, app = null } = $props();

    let dragging = $state(false);
    let resumeFile = $state<File | null>(null);
    let resumeId = $state<number | null>(app?.resume ?? null);
    let resumeError = $state("");

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
          console.log("resumeFile at submit:", resumeFile);
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
</script>

<div class="overlay">
    <div class="dialog" role="dialog" aria-modal="true" aria-labelledby="new-title">
        <h2 id="new-title">{app ? "Edit application" : "New application"}</h2>

        <form onsubmit={submit}>

          <label class="drop" class:over={dragging}
            ondragover={(e) => { e.preventDefault(); dragging = true; }}
            ondragleave={() => dragging = false}
            ondrop={(e) => {
              e.preventDefault(); dragging = false;
              resumeFile = e.dataTransfer?.files[0] ?? null;
            }}>
            <input type="file" accept=".pdf" hidden
             onchange={(e) => resumeFile = e.currentTarget.files?.[0] ?? null} />
            {#if resumeFile}
              <b>{resumeFile.name}</b>
            {:else}
              <b>Drop your resume here</b>
              <small>or click to browse · PDF</small>
            {/if}
          </label>

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
</style>
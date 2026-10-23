<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { STATUS } from "$lib/status";

    let { onclose, onsaved, app = null } = $props();

    let company = $state(app?.company ?? "");
    let position = $state(app?.position ?? "");
    let website = $state(app?.website ?? "");
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
          if (app) {
            await invoke("update_application", {
              id: app.id, company, position, website, username, applyDate, status, notes, refer, password
            });
            onsaved(app.id)
          } else {
            const id = await invoke<number>("add_application", {
                company, position, website, username, applyDate, status, notes, refer, password
            }); 
            onsaved(id);
          }
        } catch(err) {
            error = String(err);
        }
    }
</script>

<div class="overlay">
    <div class="dialog" role="dialog" aria-modal="true" aria-labelledby="new-title">
        <h2 id="new-title">{app ? "Edit application" : "New application"}</h2>

        <form onsubmit={submit}>
        <input placeholder="Company *" bind:value={company} required />
        <input placeholder="Position *" bind:value={position} required />
        <input placeholder="Website URL" bind:value={website} />
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
  .dialog { width: min(560px, 90vw); max-height: 90vh; overflow: auto; padding: 24px;
    border-radius: 16px; background: rgba(255, 255, 255, .9);
    backdrop-filter: blur(20px); box-shadow: 0 24px 64px rgba(17, 24, 39, .28); }
  h2 { margin: 0 0 16px; font-size: 20px; font-weight: 600; }
  form { display: flex; flex-direction: column; gap: 10px; }
  input, select, textarea { min-height: 44px; padding: 8px 12px;
    border: 1px solid #D1D5DB; border-radius: 8px; background: #fff; }
  .actions { display: flex; justify-content: flex-end; gap: 10px; margin-top: 8px; }
  .actions button { min-height: 44px; padding: 0 18px; border-radius: 8px;
    border: 1px solid #D1D5DB; background: #fff; cursor: pointer; }
  .actions .primary { background: var(--accent); border-color: var(--accent);
    color: #fff; font-weight: 600; }
  .error { margin: 0; color: #B91C1C; font-size: 13px; }
</style>
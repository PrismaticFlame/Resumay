<script lang="ts">
    import { openUrl } from "@tauri-apps/plugin-opener";
    import StatusPill from "./StatusPill.svelte";
    let { app, ondelete, onedit } = $props();
    let confirming = $state(false);

    const withScheme = (u: string) => /^https?:\/\//.test(u) ? u : `https://${u}`;
</script>

<div class="pane divided">
    <section>
        <div class="top">
            <StatusPill status={app.status} />
            <button class="edit" onclick={() => onedit(app)}>Edit</button>
        </div>
        <h2>{app.company}</h2>
        <p class="role">{app.position}</p>
        <p class="meta">
            <span>{app.location ?? "-"}</span>
            <span>{app.work_mode ?? "-"}</span>
            <span>{app.salary ?? "-"}</span>
        </p>
        {#if app.next_step}<p class="next">Next: {app.next_step}</p>{/if}
        <div class="links">
            <button disabled={!app.website} onclick={() => openUrl(withScheme(app.website))}>Posting ↗</button>
            <button disabled={!app.portal_url} onclick={() => openUrl(withScheme(app.portal_url))}>Portal ↗</button> 
        </div>
    </section>

    <section>
        <h3>Timeline</h3>
        <ol class="timeline">
            {#each app.timeline ?? [] as step}
                <li class:done={step.done}>
                    <i></i><span>{step.label}</span><time>{step.date}</time>
                </li>
            {:else}
                <li class="done"><i></i><span>Applied</span><time>{app.apply_date}</time></li>
            {/each}
        </ol>
    </section>

    <section>
        <div class="row">
            <h3>Resume</h3>
        </div>
        <div class="file"><b>PDF</b><span class="mono">{app.resume ?? "-"}</span><button>Open</button></div>
        <div class="file"><b>TeX</b><span class="mono">{app.tex ?? "-"}</span><button>Open</button></div>

    </section>
    
    <section>
        <h3>Portal login</h3>
        <dl>
            <dt>Portal</dt><dd>{app.portal ?? "-"}</dd>
            <dt>Website</dt><dd class="mono">{app.website ?? "—"}</dd>
            <dt>Username</dt><dd class="mono">{app.username ?? "—"}</dd>
            <!-- <dt>Password</dt>
            <dd class="pw">
                <span class="mono">{revealed ? password : "••••••••••"}</span>
                <button onclick={() => revealed =! revealed}>{revealed ? "Hide" : "Show"}</button>
            </dd> -->
        </dl>
    </section>

    <section>
        <h3>Job Description</h3>
        <p class="job_descrip">{app.job_description ?? ""}</p>
    </section>

    <section>
        <h3>Notes</h3>
        <p class="notes">{app.notes ?? "—"}</p>
        <h3>Referral</h3><p>{app.refer ?? "—"}</p>
    </section>

    
</div>

<div class="danger">
    {#if confirming}
        <span>Delete this application?</span>
        <button class="del" onclick={ondelete}>Yes, delete</button>
        <button onclick={() => confirming = false}>Cancel</button>
    {:else}
        <button class="del-ghost" onclick={() => confirming = true}>Delete</button>
    {/if}
</div>

<style>
    /* layout */
    .pane { padding: 0px; }
    section { padding: 20px 24px; display: flex; flex-direction: column; gap: 10px; }
    section p { margin: 0; }
    
    /* headings */
    h2 { margin: 10px 0 2px; font-size: 22px; font-weight: 600; }
    .role { margin: 0 0 20px; }
    h3 { margin: 0; font-size: 12px; font-weight: 600; color: var(--muted);
         text-transform: uppercase; letter-spacing: .06em; }
    section > * + h3 { margin-top: 10px; }

    /* header section */
    .top, .row { display: flex; align-items: center; justify-content: space-between; }
    .meta { display: flex; flex-wrap: wrap; gap: 6px 14px; color: var(--muted); font-size: 13px; }
    .next { color: #9A3412; font-size: 13px; font-weight: 600; }
    .mono { font-family: 'IBM Plex Mono', monospace; font-size: 13px; }

    /* timeline */
    .timeline { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 12px; }
    .timeline li { display: flex; align-items: center; gap: 12px; color: var(--muted); }
    .timeline li span { flex: 1; }
    .timeline i { width: 12px; height: 12px; border-radius: 50%; border: 2px solid #9CA3AF;
                box-sizing: border-box; flex: none; }
    .timeline li.done { color: var(--ink); }
    .timeline li.done i { border-color: var(--accent); background: var(--accent); }
    .timeline time { font-family: 'IBM Plex Mono', monospace; font-size: 12px; color: var(--muted); }

    /* resume files */
    .file { display: flex; align-items: center; gap: 12px; }
    .file b { width: 40px; height: 40px; display: grid; place-items: center; flex: none;
            border-radius: 8px; background: var(--bg); font: 600 11px 'IBM Plex Mono', monospace; }
    .file span { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }

    /* portal */
    dl { display: grid; grid-template-columns: 96px 1fr; gap: 8px 12px; align-items: center; margin: 0; }
    dt { color: var(--muted); font-size: 13px; }
    dd { margin: 0; min-width: 0; overflow-wrap: anywhere; }

    /* long text */
    .job_descrip, .notes { 
        white-space: pre-wrap;
        overflow-wrap: anywhere;
        max-height: 300px;
        overflow-y: auto; 
        padding: 10px;
        border-radius: 10px;
        outline: none;}

    /* buttons */
    section button, .edit, .danger button { min-height: 36px; padding: 0 12px; border-radius: 8px;
    border: 1px solid var(--btn-border); background: var(--btn-bg); color: var(--ink); cursor: pointer; }
    .edit, .danger button { min-height: 44px; padding: 0 14px; }   
    .links { display: flex; gap: 8px; margin-top: 4px; }
    .links button:disabled { opacity: .45; cursor: default; }

    /* delete row */    
    .danger { display: flex; align-items: center; gap: 8px; margin-top: 28px;
          padding-top: 16px; padding-left: 16px; border-top: 1px solid var(--line); }
    .del { background: #B91C1C !important; border-color: #B91C1C !important; color: #fff; }
    .del-ghost { color: var(--danger) !important; }

</style>
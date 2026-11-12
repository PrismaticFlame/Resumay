<script lang="ts">
    let { onclose }: { onclose: () => void } = $props();

    type Theme = 'light' | 'dark';

    let dialog = $state<HTMLDialogElement>();

    $effect(() => {
        dialog?.showModal();
    });

    let theme = $state<Theme>(
        (document.documentElement.dataset.theme as Theme) ?? 'light'
    );

    function setTheme(next: Theme) {
        theme = next;
        document.documentElement.dataset.theme = next;
        localStorage.setItem('theme', next);
    }

</script>

<!-- <div class="overlay" onclick={(e) => e.target === e.currentTarget && onclose()} role="presentation">
  
  <div class="panel" role="dialog" tabindex="-1" aria-modal="true" aria-label="Settings">
    <h2>Settings</h2>
    <div class="row">
      <span>Color mode</span>
      <div class="seg">
        <button class:active={theme === 'light'} onclick={() => setTheme('light')}>Light</button>
        <button class:active={theme === 'dark'} onclick={() => setTheme('dark')}>Dark</button>
      </div>
    </div>
  </div>
</div> -->

<dialog
    bind:this={dialog}
    aria-label="Settings"
    onclose={onclose}
    onclick={(e) => e.target === e.currentTarget && e.currentTarget.close()}
>
    <div class="panel">
        <h2>Settings</h2>
        <div class="row">
        <span>Color mode</span>
        <div class="seg">
            <button class:active={theme === 'light'} onclick={() => setTheme('light')}>Light</button>
            <button class:active={theme === 'dark'} onclick={() => setTheme('dark')}>Dark</button>
        </div>
        </div>
    </div>
</dialog>

<style>     
    .panel { 
        padding: 24px;
    }
    .row { 
        display: flex;
        align-items: center;
        justify-content: space-between;    
    }
    .seg {
        display: flex;
        gap: 4px;
        padding: 3px;
        background: var(--bg);
        border-radius: 8px;
    }
    .seg button { 
        border: none;
        background: transparent;
        color: var(--muted);
        padding: 6px 14px;
        border-radius: 6px;
        cursor: pointer;
        font: inherit;
    }
    .seg button.active {
        background: var(--surface);
        color: var(--ink);
        font-weight: 600;
    }
    dialog { 
        padding: 0;
        width: min(420px, 90vw);
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
</style>
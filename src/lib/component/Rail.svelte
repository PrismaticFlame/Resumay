<script lang="ts">
    import logo from "$lib/assets/resumay-logo.svg"
</script>

<nav>
    <div class="brand"><img src={logo} alt="Resumay" width="190px" height="50px"></div>
    <button class="item on">Applications</button>
    <button class="item">Resumes</button>
    <button class="item">Tailor</button>
</nav>

<style>
    nav { display: flex; flex-direction: column; gap: 4px; padding: 22px 14px;
            background: var(--surface); border-right: 1px solid var(--inline); }
    .brand { font-size: 19px; font-weight: 700; padding: 0 8px 24px; }
    .item { text-align: left; min-height: 44px; padding: 0 12px; border: none; 
            border-radius: 8px; background: transparent; color: var(--ink); cursor: pointer; }
    .item.on { background: var(--accent-weak); color: var(--accent); font-weight: 600; }
</style>

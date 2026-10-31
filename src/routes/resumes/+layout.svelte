<script lang="ts">
    let { children } = $props();
</script>

<nav>
    <a href="/">Applications</a>
    <a href="/resumes">Resumes</a>
</nav>

{@render children()}
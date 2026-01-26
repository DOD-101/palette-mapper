<script lang="ts">
    import favicon from "$lib/assets/favicon.svg";

    let { children } = $props();
</script>

<svelte:head>
    <link rel="icon" href={favicon} />
    <title>Palette Mapper Web UI</title>
    <link rel="stylesheet" href="/vars.css" />
    <link rel="stylesheet" href="/font.css" />
    <link rel="stylesheet" href="/font.css" />
    <link rel="stylesheet" href="/global.css" />
</svelte:head>

<main>
    {@render children()}
</main>

<style>
    main {
        display: flex;
        flex-direction: row;
    }
</style>

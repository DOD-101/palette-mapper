<script lang="ts">
    import Sidebar from "./sidebar.svelte";
    import Img from "./img.svelte";
    import SiteTitle from "$lib/site_title.svelte";
</script>

<div class="rail">
    <SiteTitle />
    <Sidebar />
</div>

<Img />

<style>
    .rail {
        display: flex;
        flex: none;
        flex-direction: column;
        width: max-content;
        height: 100vh;
        min-height: 0;
        background-color: color-mix(
            in srgb,
            var(--secondary) 14%,
            var(--background)
        );
        border-right: 1px solid var(--secondary);
    }
</style>

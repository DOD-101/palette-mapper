<script lang="ts">
    import { resolve } from "$app/paths";

    import SiteTitle from "$lib/site_title.svelte";
</script>

<SiteTitle />

<div class="error-container">
    <h2>404 - Page Not Found</h2>
    <p>The page you're looking for doesn't exist.</p>
    <a href={resolve("/")}>Go back to the homepage</a>
</div>

<style>
    .error-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100vh;
        width: 100%;
        text-align: center;
    }

    h2 {
        font-size: 2rem;
        margin-bottom: 1rem;
    }

    p {
        font-size: 1.2rem;
        margin-bottom: 2rem;
    }

    a {
        color: var(--accent);
        text-decoration: none;
        font-size: 1.1rem;
        padding: 0.5rem 1rem;
        border: 2px solid var(--grid-lines);
        border-radius: 4px;
        transition: all 200ms ease-in-out;

        &:hover {
            background-color: var(--accent);
            color: var(--background);
        }
    }
</style>

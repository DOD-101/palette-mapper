<script lang="ts">
    import "iconify-icon";
    import { onMount } from "svelte";

    let light_mode = $state(
        window.matchMedia("(prefers-color-scheme: light)").matches,
    );

    function changeTheme() {
        document.documentElement.dataset.theme = light_mode ? "dark" : "light";

        light_mode = !light_mode;
    }

    onMount(() => {
        document.documentElement.dataset.theme = light_mode ? "light" : "dark";
    });
</script>

<button aria-label="Change color theme" class="theme" onclick={changeTheme}>
    <iconify-icon
        icon="material-symbols:{light_mode
            ? 'dark-mode-rounded'
            : 'light-mode'}"
        width="1.5rem"
        height="1.5rem"
    >
    </iconify-icon>
</button>

<style>
    .theme {
        position: absolute;
        bottom: 10px;
        left: 10px;
        height: 2rem;
        width: 2rem;
        background-color: transparent;
        border: none;
        color: var(--text);
        transition: scale 200ms ease-in-out;

        iconify-icon {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
        }

        &:hover {
            scale: 1.1;
        }
    }
</style>

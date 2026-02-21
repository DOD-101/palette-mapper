<script lang="ts">
    import { onMount } from "svelte";
    import Sidebar from "./sidebar.svelte";
    import Img from "./img.svelte";

    let light_mode = $state(
        window.matchMedia("(prefers-color-scheme: light)").matches,
    );

    import "iconify-icon";

    function changeTheme() {
        if (light_mode) {
            document.documentElement.classList.add("dark-mode");
            document.documentElement.classList.remove("light-mode");
        } else {
            document.documentElement.classList.add("light-mode");
            document.documentElement.classList.remove("dark-mode");
        }

        light_mode = !light_mode;
    }

    onMount(() => {
        if (light_mode) {
            document.documentElement.classList.add("light-mode");
            document.documentElement.classList.remove("dark-mode");
        }
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

<h1>Palette Mapper</h1>

<Sidebar />

<Img />

<style>
    h1 {
        position: absolute;
        left: 0;
        top: 0;

        margin: 2rem;
    }

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

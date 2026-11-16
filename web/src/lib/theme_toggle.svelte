<script lang="ts">
  import "iconify-icon";
  import { onMount } from "svelte";

  let light_mode = $state(false);

  function apply_theme() {
    document.documentElement.dataset.theme = light_mode ? "light" : "dark";
  }

  function change_theme() {
    light_mode = !light_mode;
    apply_theme();
  }

  onMount(() => {
    light_mode = window.matchMedia("(prefers-color-scheme: light)").matches;
    apply_theme();
  });
</script>

<button aria-label="Change color theme" class="theme" onclick={change_theme}>
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

<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { img_data } from "./img.svelte.ts";
  import "iconify-icon";

  let img: string | undefined = $state();
  let img_orig: string | undefined = $state();

  let overlay: boolean = $state(false);
  let showOriginal: boolean = $state(false);

  $effect(() => {
    if (img_data.converted.data.size !== 0) {
      const old = untrack(() => img);
      if (old) {
        URL.revokeObjectURL(old);
      }
      img = URL.createObjectURL(img_data.converted.data);
    }
  });

  $effect(() => {
    if (img_data.original.data.size !== 0) {
      const old = untrack(() => img_orig);
      if (old) {
        URL.revokeObjectURL(old);
      }
      img_orig = URL.createObjectURL(img_data.original.data);
    }
  });

  onDestroy(() => {
    if (img) {
      URL.revokeObjectURL(img);
    }

    if (img_orig) {
      URL.revokeObjectURL(img_orig);
    }
  });

  function img_click() {
    if (overlay) {
      return;
    }

    overlay = true;
    showOriginal = false;
  }

  function close_overlay() {
    overlay = false;
    showOriginal = false;
  }

  function close_on_background(e: MouseEvent) {
    if (!overlay) {
      return;
    }

    // The image itself and the overlay controls must not close the overlay.
    if (
      e.target instanceof Element &&
      e.target.closest(".img-preview-btn, .overlay-action")
    ) {
      return;
    }

    close_overlay();
  }

  function download() {
    if (!img) {
      return;
    }

    const download_link = document.createElement("a");
    download_link.href = img;
    download_link.download = img_data.converted.file_name;
    download_link.style.opacity = "0";
    download_link.id = "dl-link";

    document.body.appendChild(download_link);
    download_link.click();
    document.body.removeChild(download_link);
  }
</script>

<svelte:window onclick={close_on_background} />

{#if overlay}
  <div class="backdrop"></div>
{/if}

<div class={["container-outer", overlay && "overlay"]}>
  {#if img}
    <div class={["container-inner", overlay && "overlay"]}>
      <button
        onclick={img_click}
        class={["img-preview-btn", overlay && "overlay"]}
      >
        <div
          class="img-container"
          class:side-by-side={showOriginal && img_orig}
        >
          {#if showOriginal && img_orig}
            <img
              alt="Original"
              class={["img-preview", overlay && "overlay"]}
              src={img_orig}
            />
          {/if}
          <img
            alt="Converted"
            class={["img-preview", overlay && "overlay"]}
            src={img}
          />
        </div>
      </button>

      {#if overlay}
        <button
          title="Download"
          class="overlay-action"
          id="download"
          onclick={download}
        >
          <iconify-icon icon="material-symbols:download-for-offline-rounded"
          ></iconify-icon>
        </button>
        <button
          aria-label="Before and after side by side"
          title="Before and after"
          class={["overlay-action", showOriginal && "active"]}
          id="side-by-side"
          onclick={() => (showOriginal = !showOriginal)}
        >
          <iconify-icon icon="material-symbols:side-navigation"></iconify-icon>
        </button>
      {/if}
    </div>
  {:else}
    <!-- TODO: make this show the image once uploaded and provide a better UI for uploading -->
    <div class="img-placeholder">
      <div>
        <p>Get started by <a href="#img-upload">uploading</a> an image.</p>
        <p>Your mapped image will appear here.</p>
      </div>
    </div>
  {/if}
</div>

<style>
  .container-outer {
    position: relative;
    display: flex;
    flex: 1;
    align-items: center;
    justify-content: center;
    min-width: 0;
    z-index: 10;
    padding: 2rem;

    /* Definite size so the image can size itself against the pane without
       depending on its (shrink-wrapped) button parent. */
    container-type: size;

    .container-inner {
      position: relative;
      transition:
        scale 100ms ease-in-out,
        margin-bottom 100ms ease-in-out;
      width: fit-content;
      max-width: 100cqw;
      display: block;
      text-align: center;

      &:hover {
        scale: 1.01;
      }

      &.overlay {
        scale: 1.04;
        margin-bottom: 2rem;
      }
    }

    .img-placeholder {
      display: flex;
      align-items: center;
      justify-content: center;
      padding: 5rem;
      border: 2px dashed var(--accent);
      border-radius: 16px;

      p {
        text-align: center;
      }
    }
  }

  .img-preview-btn {
    display: block;
    width: fit-content;
    max-width: 100cqw;
    height: fit-content;
    padding: 0;
    background-color: transparent;
    border-radius: 0;
    border: none;
  }

  .img-preview {
    display: block;
    width: auto;
    height: auto;
    max-width: 100cqw;
    max-height: 90cqh;
    object-fit: contain;
  }

  .img-container.side-by-side .img-preview {
    max-width: 50cqw;
    min-width: 0;
  }

  .overlay-action {
    background-color: transparent;
    border: none;
    color: var(--text);
    font-size: 1.5rem;
    margin-top: 0.5rem;
    transition: scale 200ms ease-in-out;

    &:hover {
      scale: 1.1;
    }

    &.active {
      color: var(--accent);
    }
  }

  .img-container {
    display: flex;
    width: fit-content;
    height: fit-content;
    gap: 0;
  }

  .backdrop {
    position: absolute;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: black;
    opacity: 0.4;
    z-index: 1;
  }
</style>

<script lang="ts">
  import { notifications, NotificationLevel, UiNotification } from "$lib";
  import { img_data } from "./img.svelte.ts";

  import Row from "$lib/row.svelte";
  import FileUpload from "$lib/file_upload.svelte";

  import Palette from "./palette.svelte";

  import "iconify-icon";

  import * as wasm from "../../wasm/pkg";

  let algorithm = $state("");

  let image_files: FileList | undefined = $state();

  let palette: string = $state("");
  let palette_name: string = $state("");

  let locked = $state(false);
  let loading = $state(false);

  async function load_algorithms() {
    await wasm.default();

    algorithm = wasm.algorithms()[0];

    return wasm.algorithms();
  }

  $effect(() => {
    const file = image_files?.item(0);

    if (!file) {
      return;
    }

    const image_reader = new FileReader();

    image_reader.onload = (reader_ev) => {
      // Ignore results for a file that is no longer the current selection.
      if (image_files?.item(0) !== file) {
        return;
      }

      const bytes = reader_ev.target?.result;

      if (!(bytes instanceof ArrayBuffer)) {
        return;
      }

      img_data.original.file_name = file.name;
      img_data.original.data = new Blob([bytes]);
    };

    image_reader.onerror = () => {
      if (image_files?.item(0) !== file) {
        return;
      }

      notifications.push_notification(
        new UiNotification(
          NotificationLevel.Error,
          "Failed to read the uploaded image.",
        ),
      );
    };

    image_reader.readAsArrayBuffer(file);
  });

  async function submit(
    e: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement },
  ) {
    const start = performance.now();
    loading = true;

    if (img_data.original.data.size === 0) {
      loading = false;
      notifications.push_notification(
        new UiNotification(
          NotificationLevel.Error,
          "No image uploaded. Nothing to convert.",
        ),
      );

      return;
    }

    if (!palette) {
      loading = false;
      notifications.push_notification(
        new UiNotification(
          NotificationLevel.Error,
          "No palette set. What colors should be used?",
        ),
      );

      return;
    }

    animateSubmit(e);

    notifications.push_notification(
      new UiNotification(
        NotificationLevel.Info,
        "Mapping image. This may take a sec.",
      ),
    );

    if (import.meta.env.DEV) {
      console.log(
        "Converting image with params:\n(algorithm)",
        algorithm,
        "\n(palette)",
        palette,
        "\n(image)",
        image_files,
      );
    }

    const worker = new Worker(new URL("./wasm.worker.ts", import.meta.url), {
      type: "module",
    });

    worker.onmessage = (e) => {
      worker.terminate();
      loading = false;

      if (import.meta.env.DEV) {
        console.log(`Mapped image. Took: ${performance.now() - start}ms`);
      }

      const originalNameNoExtension = img_data.original.file_name.replace(
        /\.[^/.]+$/,
        "",
      );

      img_data.converted.data = new Blob([e.data as BlobPart], {
        type: "image/png",
      });

      img_data.converted.file_name = `${originalNameNoExtension}-${palette_name}.png`;
    };

    worker.onerror = (e) => {
      worker.terminate();
      loading = false;
      notifications.push_notification(
        new UiNotification(
          NotificationLevel.Error,
          `Failed to map image: ${e.message}`,
        ),
      );
    };

    worker.postMessage({
      image_bytes: await img_data.original.data.bytes(),
      palette: palette,
      algorithm: algorithm,
    });
  }

  function animateSubmit(
    e: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement },
  ) {
    locked = true;

    const ripple = document.createElement("div");

    ripple.classList.add("ripple");

    const rect = e.currentTarget.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    ripple.style.left = `${x}px`;
    ripple.style.top = `${y}px`;

    e.currentTarget.appendChild(ripple);

    setTimeout(() => {
      ripple.remove();
      locked = false;
    }, 400);
  }

  function track_mouse(
    e: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement },
  ) {
    if (locked) {
      return;
    }

    const rect = e.currentTarget.getBoundingClientRect();

    e.currentTarget.style.setProperty(
      "--mouse-x",
      `${e.clientX - rect.left}px`,
    );
    e.currentTarget.style.setProperty("--mouse-y", `${e.clientY - rect.top}px`);
  }
</script>

<div class="sidebar">
  <div id="inputs">
    <Row>
      <h5 class="heading">Image</h5>
      <FileUpload
        label={img_data.original.file_name || "Upload"}
        bind:files={image_files}
        classNames="input-elm upload-btn"
        title="Select image to map."
        accept="image/*"
      />
    </Row>

    <Palette bind:palette bind:palette_name />

    <Row>
      <h5 class="heading">Algorithm</h5>
      {#await load_algorithms()}
        <p class="loading">Loading…</p>
      {:then algos}
        <div class="algorithm-select">
          <select bind:value={algorithm} aria-label="Algorithm">
            {#each algos as algo}
              <option value={algo}>{algo}</option>
            {/each}
          </select>

          <iconify-icon
            icon="material-symbols:arrow-drop-down-rounded"
            width="1.5rem"
            height="1.5rem"
          >
          </iconify-icon>
        </div>
      {/await}
    </Row>

    <Row>
      <button
        onclick={submit}
        onmousemove={track_mouse}
        id="submit"
        class="input-elm"
        disabled={loading}
      >
        {#if loading}
          <iconify-icon icon="svg-spinners:90-ring-with-bg"></iconify-icon>
        {:else}
          Map Image!
        {/if}
      </button>
    </Row>
  </div>
</div>

<style>
  .sidebar {
    display: flex;
    flex: 1;
    flex-direction: column;
    width: 100%;
    min-height: 0;
    overflow-y: auto;
    contain: inline-size;

    #inputs {
      display: flex;
      flex: 1;
      flex-direction: column;
      width: 100%;
      min-height: 0;
      padding: 0 1rem 2rem;
      box-sizing: border-box;
      color: var(--text);

      border-radius: 8px;

      :global(.row) {
        flex: none;
      }

      :global(.palette-row) {
        display: flex;
        flex: 1;
        flex-direction: column;
        min-height: 20rem;
      }

      :global(.upload-btn) {
        margin-inline: 0;
      }

      :global(.input-elm) {
        width: 100%;
        height: 3rem;
        /* margin: 0 1rem; */
        padding: 0;
        transition:
          background-color 200ms ease-in-out,
          scale 200ms ease-in-out;
        border: 2px solid var(--border);
        border-radius: 4px;
        box-sizing: border-box;
      }

      :global(.heading) {
        margin: 1rem 0 0.4rem;
        font-size: 2rem;
        line-height: 1.2;
      }

      .loading {
        margin: 0;
        padding: 0.25rem 0;
      }

      .algorithm-select {
        position: relative;
        width: 100%;

        select {
          width: 100%;
          height: 3rem;
          padding: 0 2.75rem 0 0.85rem;
          appearance: none;
          border: 2px solid var(--border);
          border-radius: 4px;
          background-color: var(--surface-control);
          color: var(--text);
          font: inherit;
          box-sizing: border-box;
          cursor: pointer;
          transition:
            background-color 200ms ease-in-out,
            border-color 200ms ease-in-out;

          &:hover {
            background-color: var(--surface-control-hover);
          }

          &:focus-visible {
            outline: 2px solid var(--accent);
            outline-offset: 2px;
          }

          option {
            background-color: var(--surface);
            color: var(--text);
          }
        }

        iconify-icon {
          position: absolute;
          top: 50%;
          right: 0.75rem;
          transform: translateY(-50%);
          color: var(--text);
          pointer-events: none;
        }
      }

      #submit {
        --submit-glow: var(--accent);

        position: relative;
        background-color: transparent;
        font-weight: bold;
        color: var(--text);
        cursor: pointer;
        overflow: hidden;

        border: 2px solid var(--border);
        border-radius: 4px;
        box-sizing: border-box;
        margin: 2rem auto;
        height: 2.5rem;

        transition: all 200ms ease-in-out;

        &:hover {
          scale: 1.02;

          &::after {
            opacity: 1;
          }
        }

        &:disabled {
          --submit-glow: var(--confirm);
        }

        &::after {
          opacity: 0;
          transition: opacity 400ms ease-in-out;
          content: "";
          height: 100%;
          width: 100%;
          position: absolute;
          top: 0;
          left: 0;
          z-index: -1;
          background-image: radial-gradient(
            circle at var(--mouse-x) var(--mouse-y),
            var(--submit-glow),
            transparent 40%
          );
        }
      }
    }
  }

  :global(.ripple) {
    position: absolute;
    background-image: radial-gradient(
      circle,
      var(--submit-glow) 0%,
      transparent 40%
    );
    width: 20px;
    aspect-ratio: 1;
    z-index: -1;
    border-radius: 50%;
    transform: scale(0);
    animation: 500ms ease-in 0ms 1 running ripple-effect;
  }

  @keyframes ripple-effect {
    from {
      transform: scale(0);
    }
    40% {
      transform: scale(40);
      opacity: 1;
    }
    to {
      transform: scale(40);
      opacity: 0;
    }
  }
</style>

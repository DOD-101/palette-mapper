<script lang="ts">
    import { untrack } from "svelte";
    import { img_data } from "./img.svelte.ts";
    import "iconify-icon";

    let img: string | undefined = $state();
    let img_orig: string | undefined = $state();

    let overlay: boolean = $state(false);
    let showOriginal: boolean = $state(false);

    $effect(() => {
        if (img_data.converted.data.size != 0) {
            const old = untrack(() => img);
            if (old) {
                URL.revokeObjectURL(old);
            }
            img = URL.createObjectURL(img_data.converted.data);
        }
    });

    $effect(() => {
        if (img_data.original.data.size != 0) {
            const old = untrack(() => img_orig);
            if (old) {
                URL.revokeObjectURL(old);
            }
            img_orig = URL.createObjectURL(img_data.original.data);
        }
    });

    function img_click() {
        overlay = !overlay;
        showOriginal = false;
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
                <div class="img-container">
                    {#if showOriginal && img_orig}
                        <img
                            alt="Original"
                            class={["img-preview", overlay && "overlay"]}
                            style:width="50%"
                            src={img_orig}
                        />
                    {/if}
                    <img
                        alt="Converted"
                        class={["img-preview", overlay && "overlay"]}
                        style:width={showOriginal ? "50%" : "100%"}
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
                    <iconify-icon
                        icon="material-symbols:download-for-offline-rounded"
                    ></iconify-icon>
                </button>
                <button
                    aria-label="Before and after side by side"
                    title="Before and after"
                    class={["overlay-action", showOriginal && "active"]}
                    id="side-by-side"
                    onclick={() => (showOriginal = !showOriginal)}
                >
                    <iconify-icon icon="material-symbols:side-navigation"
                    ></iconify-icon>
                </button>
            {/if}
        </div>
    {:else}
        <div class="img-placeholder">
            <p>Mapped image will appear here.</p>
        </div>
    {/if}
</div>

<style>
    .container-outer {
        position: relative;
        display: block;
        width: 100%;
        z-index: 10;

        .container-inner,
        .img-placeholder {
            position: relative;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
        }

        .container-inner {
            transition:
                scale 100ms ease-in-out,
                margin-bottom 100ms ease-in-out;
            width: 80vw;
            height: 90vh;
            display: block;
            transform-origin: top left;

            &:hover {
                scale: 1.01;
            }

            &.overlay {
                scale: 1.04;
                margin-bottom: 2rem;
            }
        }

        .img-placeholder {
            border: 5px dashed var(--accent);
            width: 20vw;
            height: 20vh;
            border-radius: 16px;

            p {
                position: absolute;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                margin: 0;
            }
        }
    }

    .img-preview-btn,
    .img-preview {
        display: block;
        background-color: transparent;
        border-radius: 0;
        border: none;

        width: 100%;
        height: 100%;
        object-fit: contain;
    }

    .overlay-action {
        background-color: transparent;
        border: none;
        color: var(--text);
        font-size: 1.5rem;
        margin-top: 0rem;
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
        width: 100%;
        height: 100%;
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

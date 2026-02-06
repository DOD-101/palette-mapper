<script lang="ts">
    import {
        notifications,
        NotificationLevel,
        UiNotification,
    } from "$lib/notifications.svelte.ts";
    import { converted_img } from "./sidebar.svelte.ts";

    import Row from "$lib/row.svelte";
    import FileUpload from "$lib/file_upload.svelte";

    import Palette from "./palette.svelte";

    import * as wasm from "../../wasm/pkg";

    let algorithm = $state("");

    let image_files: FileList | undefined = $state();
    let image_bytes: Uint8Array | undefined = $state();
    let image_label: string = $derived.by(() => {
        if (image_files === undefined) {
            return "Upload";
        }

        let file = image_files.item(0);

        if (file === null) {
            return "Upload";
        }

        return file.name;
    });

    let palette: string = $state("");

    const wasmInit = async () => {
        await wasm.default();

        // TODO: Figure out a better way to do this
        algorithm = wasm.algorithms()[0];

        return wasm.algorithms();
    };

    $effect(() => {
        console.log(image_files);
        if (!image_files) {
            return;
        }

        let file = image_files.item(0);
        if (file) {
            const image_reader = new FileReader();

            image_reader.onload = (reader_ev) => {
                if (!reader_ev.target) {
                    return;
                }

                console.log(reader_ev);
                const bytes = new Uint8Array(
                    reader_ev.target.result as ArrayBuffer,
                );

                image_bytes = bytes;
            };

            image_reader.onerror = (err) => {
                notifications.push_notification(
                    new UiNotification(
                        NotificationLevel.Error,
                        "Failed to read converted image: " + err,
                    ),
                );
            };

            image_reader.readAsArrayBuffer(file);
        }
    });

    function submit(
        e: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement },
    ) {
        const start = performance.now();

        if (!image_bytes) {
            notifications.push_notification(
                new UiNotification(
                    NotificationLevel.Error,
                    `No image uploaded. Nothing to convert.`,
                ),
            );

            return;
        }

        if (!palette) {
            notifications.push_notification(
                new UiNotification(
                    NotificationLevel.Error,
                    `No palette set. What colors should be used?`,
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
        console.log(
            "Converting image with params:\n(algorithm)",
            algorithm,
            "\n(palette)",
            palette,
            "\n(image)",
            image_files,
        );

        let worker = new Worker(new URL("./wasm.worker.ts", import.meta.url), {
            type: "module",
        });

        worker.onmessage = (e) => {
            console.log(`Mapped image. Took: ${performance.now() - start}ms`);

            converted_img.data = new Blob([e.data as BlobPart], {
                type: "image/png",
            });
        };

        worker.postMessage({
            image_bytes: image_bytes,
            palette: palette,
            algorithm: algorithm,
        });
    }

    function animateSubmit(
        e: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement },
    ) {
        locked = true;

        let ripple = document.createElement("div");

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

    let locked = $state(false);

    function submitMouseover(
        e: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement },
    ) {
        let submit_btn = e.currentTarget;

        if (!submit_btn) {
            return;
        }

        submit_btn.onmousemove = (e) => {
            const rect = submit_btn.getBoundingClientRect(),
                x = e.clientX - rect.left,
                y = e.clientY - rect.top;

            if (!locked) {
                submit_btn.style.setProperty("--mouse-x", `${x}px`);
                submit_btn.style.setProperty("--mouse-y", `${y}px`);
            }
        };
    }
</script>

<div class="sidebar">
    <div id="inputs">
        <Row>
            <h5 class="heading">Algorithm</h5>
            <select
                name="algorithm"
                class="input-elm algorithm"
                id="algorithms"
                bind:value={algorithm}
                required
            >
                {#await wasmInit()}
                    <option disabled>Loading…</option>
                {:then algos}
                    {#each algos as algo}
                        <option value={algo}>{algo}</option>
                    {/each}
                {/await}
            </select>
        </Row>

        <Palette bind:palette />

        <Row>
            <h5 class="heading">Image</h5>
            <FileUpload
                label={image_label}
                bind:files={image_files}
                classNames="input-elm"
                title="Select image to map."
                accept="image/*"
            />
        </Row>

        <button
            onclick={(e) => submit(e)}
            onmousemove={submitMouseover}
            id="submit"
            class="input-elm">Map Image!</button
        >
    </div>
</div>

<style>
    .sidebar {
        display: flex;
        width: fit-content;
        height: 100vh;
        align-items: center;
        overflow: visible;

        #inputs {
            display: flex;
            flex-direction: column;
            padding: 5px;
            width: 12vw;

            max-width: 200px;
            color: var(--text);

            border-radius: 8px;

            padding: 1rem;

            :global(.input-elm) {
                width: 10vw;
                height: 1.2lh;
                padding: 0;
                max-width: 200px;
                transition:
                    background-color 200ms ease-in-out,
                    scale 200ms ease-in-out;
                border: 2px solid var(--grid-lines);
                border-radius: 4px;
                margin: 0;
                box-sizing: border-box;
            }

            :global(.heading) {
                margin: 0;
                font-size: 1rem;
            }

            #submit {
                position: relative;
                background-color: transparent;
                font-weight: bold;
                color: var(--text);
                cursor: pointer;
                overflow: hidden;

                border: 2px solid var(--grid-lines);
                border-radius: 4px;
                box-sizing: border-box;
                width: calc(10vw - 1rem * 2);
                margin: 1rem auto;
                height: 2rem;

                @media (max-width: 1400px) {
                    width: 10vw;
                    margin: 1rem initial;
                }

                transition: all 200ms ease-in-out;

                &:hover {
                    scale: 1.02;

                    &::after {
                        opacity: 1;
                    }
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
                        var(--confirm),
                        transparent 40%
                    );
                }
            }

            .algorithm {
                background-color: var(--secondary);
                color: inherit;
                border: none;
                border-radius: 3px;
                padding-left: 6px;
                box-sizing: border-box;

                &:hover {
                    background-color: var(--accent);
                }
            }
        }
    }

    :global(.ripple) {
        position: absolute;
        background-image: radial-gradient(
            circle,
            var(--confirm) 0%,
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

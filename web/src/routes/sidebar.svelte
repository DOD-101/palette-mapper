<script lang="ts">
    import { UiNotification, NotificationLevel } from "$lib/ui_notification";
    import { converted_img } from "./sidebar.svelte.ts";

    import Row from "$lib/row.svelte";
    import FileUpload from "$lib/file_upload.svelte";

    import Palette from "./palette.svelte";

    import * as wasm from "../../wasm/pkg/";

    let algorithm = $state("");

    let image_files: FileList | undefined = $state();
    let image_bytes: Uint8Array | undefined = $state();
    let image_label: string = $derived.by(() => {
        if (image_files === undefined) {
            return "Image";
        }

        let file = image_files.item(0);

        if (file === null) {
            return "Image";
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
                new UiNotification(
                    NotificationLevel.Error,
                    "Failed to read image: " + err,
                ).display();
            };

            image_reader.readAsArrayBuffer(file);
        }
    });

    function submit() {
        const start = performance.now();

        if (!image_bytes) {
            new UiNotification(NotificationLevel.Debug, `No image bytes set`);

            return;
        }

        if (!palette) {
            new UiNotification(NotificationLevel.Debug, `No palette set`);

            return;
        }

        console.log(
            "Converting image with params:\n(algorithm)",
            algorithm,
            "\n(palette)",
            palette,
            "\n(image)",
            image_files,
        );
        const converted = wasm.map_image(image_bytes, palette, algorithm);

        converted_img.data = new Blob([converted as BlobPart], {
            type: "image/png",
        });

        const end = performance.now();

        console.log(`Converted image. Took: ${end - start}ms`);
    }
</script>

<div class="sidebar">
    <form id="inputs">
        <Row>
            <label for="algorithm">Algorithm</label>

            <select
                name="algorithm"
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
            <FileUpload label={image_label} bind:files={image_files} />
        </Row>

        <button onclick={() => submit()} id="submit">Map Image!</button>
    </form>
</div>

<style>
    .sidebar {
        display: flex;
        width: fit-content;
        height: 100vh;
        align-items: center;

        overflow: hidden;

        #inputs {
            display: flex;
            flex-direction: column;
            padding: 5px;
            width: 12vw;

            max-width: 200px;
            color: var(--text);

            border-radius: 8px;

            padding: 1rem;

            #submit {
                background-color: var(--primary);
                font-weight: bold;
                color: var(--text-alt);
                cursor: pointer;

                border: none;
                border-radius: 3px;
                margin: 0.5rem;

                transition: all 200ms ease-in-out;
                &:hover {
                    background-color: rgb(var(--confirm));
                    box-shadow: 5px 20px 50px 5px rgba(var(--confirm), 0.5);
                    scale: 1.02;
                }
            }

            :global(select) {
                background-color: var(--secondary);
                color: inherit;
                border: none;
                border-radius: 3px;
                margin: 0.5rem;
                max-width: 11vw;
            }
        }
    }
</style>

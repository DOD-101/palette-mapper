<script lang="ts">
    import { untrack } from "svelte";

    import Row from "$lib/row.svelte";
    import FileUpload from "$lib/file_upload.svelte";
    import OptionCard from "$lib/option_card.svelte";
    import type { Option } from "$lib/option";
    import * as wasm from "../../wasm/pkg/";
    import PaletteSearch from "./palette_search.svelte";
    import { notifications, UiNotification, NotificationLevel } from "$lib";

    let {
        palette = $bindable(),
        palette_name = $bindable(""),
    }: { palette: string; palette_name: string } = $props();

    const uploaded_value = "uploaded::palette";

    let pal_files: FileList | undefined = $state();
    let pal_select = $state("");
    let uploaded_contents: string | null = $state(null);
    let uploaded_colors: string[] = $state([]);

    let pal_label: string = $derived.by(() => {
        if (pal_files === undefined) {
            return "Upload";
        }

        let file = pal_files.item(0);

        if (file === null) {
            return "Upload";
        }

        return file.name;
    });

    let uploaded_option: Option | undefined = $derived.by(() => {
        let file = pal_files?.item(0);

        if (!file || uploaded_contents === null) {
            return undefined;
        }

        return {
            value: uploaded_value,
            label: file.name.replace(/\.[^/.]+$/, "") || file.name,
            badge: "Custom",
            colors: uploaded_colors,
        };
    });

    /** Drop the uploaded palette and deselect it if it was in use */
    function discard_upload() {
        pal_files = undefined;
        uploaded_contents = null;
        uploaded_colors = [];

        if (pal_select === uploaded_value) {
            pal_select = "";
        }
    }

    await wasm.default();

    /** Parse an uploaded palette into CSS colors for the preview */
    function palette_colors(contents: string): string[] {
        try {
            const raw = JSON.parse(contents);

            if (!Array.isArray(raw)) {
                return [];
            }

            return raw.flatMap((color) => {
                if (typeof color === "string") {
                    return [color];
                }

                if (!Array.isArray(color) || color.length < 3) {
                    return [];
                }

                const [r, g, b, a] = color;

                return [
                    a === undefined
                        ? `rgb(${r}, ${g}, ${b})`
                        : `rgba(${r}, ${g}, ${b}, ${a / 255})`,
                ];
            });
        } catch {
            return [];
        }
    }

    /** Read the uploaded palette and make it the selected palette */
    $effect(() => {
        const file = pal_files?.item(0);
        // Read once so a newer selection made while the file is loading wins.
        const selection_at_start = untrack(() => pal_select);

        uploaded_contents = null;
        uploaded_colors = [];

        if (!file) {
            return;
        }

        const palette_reader = new FileReader();

        /** Ignore results for a file that is no longer the current selection */
        function is_current_file(): boolean {
            return pal_files?.item(0) === file;
        }

        palette_reader.onload = (reader_ev) => {
            if (!is_current_file()) {
                return;
            }

            const contents = reader_ev.target?.result;

            if (typeof contents !== "string") {
                return;
            }

            if (!wasm.is_valid_palette(contents)) {
                notifications.push_notification(
                    new UiNotification(
                        NotificationLevel.Error,
                        "The uploaded palette is invalid.",
                    ),
                );

                discard_upload();

                return;
            }

            uploaded_contents = contents;
            uploaded_colors = palette_colors(contents);

            if (pal_select === selection_at_start) {
                pal_select = uploaded_value;
            }
        };

        palette_reader.onerror = () => {
            if (!is_current_file()) {
                return;
            }

            notifications.push_notification(
                new UiNotification(
                    NotificationLevel.Error,
                    "Failed to read the uploaded palette.",
                ),
            );

            discard_upload();
        };

        palette_reader.readAsText(file);
    });

    /** Resolve the current selection into the palette string to map with */
    $effect(() => {
        palette = "";
        palette_name = "";

        if (pal_select === uploaded_value) {
            if (uploaded_contents === null) {
                return;
            }

            palette = uploaded_contents;
            // fallback name since we don't know the name of the palette
            palette_name = "mapped";

            return;
        }

        if (pal_select.length > 0) {
            palette = wasm.pal_from_base(wasm.from_base_name(pal_select));
            palette_name = pal_select;
        }
    });
</script>

<Row classNames="palette-row">
    <h5 class="heading">Palette</h5>

    <PaletteSearch bind:selected={pal_select} />

    <p class="or">or</p>

    <div class="row-direction">
        {#if uploaded_option}
            <div class="uploaded-palette">
                <OptionCard option={uploaded_option} bind:value={pal_select} />

                <button type="button" class="remove" onclick={discard_upload}>
                    click to remove
                </button>
            </div>
        {:else}
            <FileUpload
                label={pal_label}
                bind:files={pal_files}
                classNames="input-elm"
                title="Select palette to use from file."
                accept="application/json"
            />
        {/if}
    </div>
</Row>

<style>
    .row-direction {
        position: relative;
        display: flex;
        flex-direction: row;
        margin: 0;
        width: 100%;
    }

    .uploaded-palette {
        display: flex;
        flex-direction: column;
        width: 100%;
    }

    .remove {
        align-self: flex-end;
        margin: 0;
        padding: 0;
        border: none;
        background: none;
        color: var(--text);
        font: inherit;
        font-size: 0.75rem;
        text-decoration: underline;
        opacity: 0.75;
        cursor: pointer;
        transition: opacity 150ms ease-in-out;

        &:hover {
            opacity: 1;
        }
    }

    .or {
        text-align: center;
        margin: 0.25rem 0;
    }
</style>

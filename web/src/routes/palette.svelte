<script lang="ts">
    import Row from "$lib/row.svelte";
    import FileUpload from "$lib/file_upload.svelte";
    import * as wasm from "../../wasm/pkg/";

    let { palette = $bindable() }: { palette: string } = $props();

    let pal_files: FileList | undefined = $state();
    let pal_label: string = $derived.by(() => {
        if (pal_files === undefined) {
            return "Palette";
        }

        let file = pal_files.item(0);

        if (file === null) {
            return "Palette";
        }

        return file.name;
    });

    let pal_search = $state("");

    let pal_select = $state("");

    function search_theme(term: string) {
        // smart-case
        if (pal_search.toLowerCase() == pal_search) {
            term = term.toLowerCase();
        }

        return term.includes(pal_search);
    }

    // TODO: Add selection of pal from palettes/
    let pal_upload = $state(false);

    await wasm.default();

    /** Read inputted palette from the given file */
    $effect(() => {
        if (!pal_upload) {
            if (pal_select.length === 0) {
                return;
            }

            if (pal_select.startsWith("base16::")) {
                palette = wasm.pal_from_base16(
                    wasm.from_base_16_name(pal_select.slice(8)),
                );
            } else {
                palette = wasm.pal_from_base24(
                    wasm.from_base_24_name(pal_select.slice(8)),
                );
            }

            return;
        }

        let file = pal_files?.item(0);
        if (file) {
            const palette_reader = new FileReader();

            palette_reader.onload = (reader_ev) => {
                if (reader_ev.target == null) {
                    return;
                }

                const contents = reader_ev.target.result;

                if (typeof contents !== "string") {
                    return;
                }

                palette = contents;

                console.log(palette);
            };

            palette_reader.readAsText(file);
        }
    });
</script>

<Row>
    <div id="row-direction">
        <input
            type="checkbox"
            bind:checked={pal_upload}
            title="Upload your own palette file."
        />
        {#if pal_upload}
            <FileUpload label={pal_label} files={pal_files} />
        {:else}
            <div>
                <input type="search" id="test" bind:value={pal_search} />
                <select placeholder="Search themes" bind:value={pal_select}>
                    {#each wasm.base16() as base16}
                        {#if search_theme(base16)}
                            <option value="base16::{base16}"
                                >{base16} (Base16)</option
                            >
                        {/if}
                    {/each}

                    {#each wasm.base24() as base24}
                        {#if search_theme(base24)}
                            <option value="base24::{base24}"
                                >{base24} (Base24)</option
                            >
                        {/if}
                    {/each}
                </select>
            </div>
        {/if}
    </div>
</Row>

<style>
    input[type="checkbox"] {
        border: none;
        min-width: 1rem;
        max-width: 1rem;
    }

    input[type="search"] {
        background-color: var(--secondary);
        border: none;
        border-radius: 6px;
        color: inherit;
        width: 8.5vw;
        margin: 0.5rem;
    }

    select {
        width: 8.5vw;
    }

    #row-direction {
        display: flex;
        flex-direction: row;

        :global(#file_upload_label) {
            width: 10vw;
        }
    }
</style>

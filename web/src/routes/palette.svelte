<script lang="ts">
    import Row from "$lib/row.svelte";
    import FileUpload from "$lib/file_upload.svelte";
    import * as wasm from "../../wasm/pkg/";
    import ClearBtn from "$lib/clear_btn.svelte";

    let { palette = $bindable() }: { palette: string } = $props();

    /** Where we are getting the palette from */
    const palette_source = {
        Upload: "Upload",
        PreDefined: "PreDefined",
        Undecided: "Undecided",
        get() {
            if (pal_select) {
                return this.PreDefined;
            }

            if (pal_files && pal_files.length > 0) {
                return this.Upload;
            }

            return this.Undecided;
        },
    } as const;

    let pal_files: FileList | undefined = $state();
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

    let pal_search = $state("");

    let pal_select = $state("");

    function search_theme(term: string) {
        // smart-case
        if (pal_search.toLowerCase() == pal_search) {
            term = term.toLowerCase();
        }

        return term.includes(pal_search);
    }

    await wasm.default();

    /** Read inputted palette from the given file */
    $effect(() => {
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
            };

            palette_reader.readAsText(file);

            return;
        }

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
    });
</script>

<Row>
    <p class="heading">Palette</p>
    {#if palette_source.get() == "Undecided" || palette_source.get() == "Upload"}
        <div class="row-direction">
            {#if palette_source.get() == "Upload"}
                <div>
                    <ClearBtn
                        classNames="palette clear-btn"
                        onclick={() => {
                            pal_files = undefined;
                        }}
                        hidden={!(pal_files && pal_files.length > 0)}
                    />
                </div>
            {/if}
            <div>
                <FileUpload
                    label={pal_label}
                    bind:files={pal_files}
                    classNames="input-elm"
                />
            </div>
        </div>
    {/if}

    {#if palette_source.get() == "Undecided"}
        <p class="or">or</p>
    {/if}

    {#if palette_source.get() == "Undecided" || palette_source.get() == "PreDefined"}
        <div class="row-direction">
            {#if palette_source.get() == "PreDefined"}
                <ClearBtn
                    classNames="palette clear-btn"
                    onclick={() => {
                        pal_select = "";
                    }}
                />
            {/if}
            <div>
                <input
                    type="search"
                    bind:value={pal_search}
                    class="input-elm"
                />
                <select
                    placeholder="Search palettes"
                    id="base-palettes"
                    bind:value={pal_select}
                    class="input-elm"
                >
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
        </div>
    {/if}
</Row>

<style>
    input[type="search"] {
        background-color: var(--secondary);
        border: none;
        border-radius: 6px;
        color: inherit;
        margin: 0.5rem;
        transition: background-color 200ms ease-in-out;

        &:hover {
            background-color: var(--accent);
        }
    }

    .row-direction {
        display: flex;
        flex-direction: row;
        margin: 0;
        width: 100%;
    }

    .or {
        text-align: center;
        margin: 0.5rem;
    }

    .heading {
        margin: 0;
    }

    :global(.palette.clear-btn) {
        margin-top: 5px;
    }
</style>

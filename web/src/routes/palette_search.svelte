<script lang="ts">
    import ClearBtn from "$lib/clear_btn.svelte";
    import * as wasm from "../../wasm/pkg/";
    import "iconify-icon";

    let { selected = $bindable() }: { selected: string } = $props();

    let selected_pretty = $derived.by(() => {
        if (selected.match(/^base/)) {
            const base = selected.slice(4, 6);
            const name = selected.slice(14);

            return `${name} (Base${base})`;
        }

        return "";
    });

    let searched = $state("");

    await wasm.default();

    function search_theme(term: string) {
        // smart-case
        if (searched.toLowerCase() == searched) {
            term = term.toLowerCase();
        }

        return term.includes(searched);
    }

    $effect(() => {
        if (selected) searched = selected_pretty;
    });
</script>

<div class="search-container">
    <div class={[!selected && "results-visible", "search-box input-elm"]}>
        {#if !selected}
            <iconify-icon icon="material-symbols:search-rounded">
            </iconify-icon>
        {:else}
            <ClearBtn
                onclick={() => {
                    selected = "";
                    searched = "";
                }}
                size={1}
                classNames="palette-search-clear"
            />
        {/if}

        <input
            type="search"
            bind:value={searched}
            placeholder={!!selected ? selected_pretty : "Search palette"}
            disabled={!!selected}
        />
    </div>
    {#if !selected}
        <div class="input-elm results-box">
            {#each wasm.base_both() as palette}
                {#if search_theme(palette)}
                    <button
                        onclick={() => {
                            selected = `base16::${palette}`;
                        }}
                        >{palette.slice(6)} (Base{palette.slice(4, 6)})</button
                    >
                {/if}
            {/each}
        </div>
    {/if}
</div>

<style>
    .search-box {
        display: inline-block;
        position: relative;
        background-color: var(--secondary);
        border-radius: 6px;
        margin: 0.25rem 0.5rem 0rem 0.5rem;
        border-bottom: none !important;

        &.results-visible {
            border-radius: 6px 6px 0px 0px;
        }

        input {
            border: none;
            background-color: transparent;
            margin-left: 1.1rem;
            color: var(--text);
            box-sizing: border-box;
            width: calc(100% - 1.1rem);
            transition: scale 200ms ease-in-out;

            &:focus {
                outline: none;
                box-shadow: none;
            }
        }

        iconify-icon {
            position: absolute;
            transition: scale 200ms ease-in-out;

            top: 50%;
            left: 2px;

            transform: translateY(-50%);
        }

        :global(.palette-search-clear) {
            position: absolute;
            top: 50%;
            left: 2px;
            background-color: transparent;

            transform: translateY(-50%);
        }

        &:hover iconify-icon {
            scale: 1.05;
        }
    }

    .results-box {
        background-color: var(--primary);
        border-radius: 0px 0px 3px 3px;
        border-top: none !important;
        box-sizing: border-box;
        height: initial !important;
        margin: 0rem 0.5rem;
        max-height: 10rem;
        overflow-y: scroll;
        padding: 2px !important;

        scrollbar-color: var(--accent) transparent;

        button {
            overflow-x: hidden;
            overflow-wrap: unset;
            white-space: pre;
            margin: 2px;
            width: calc(100% - 2px * 2);
            color: var(--text);

            border: none;
            border-radius: 3px;
            background-color: var(--secondary);

            &:hover {
                background-color: var(--accent);
            }
        }
    }
</style>

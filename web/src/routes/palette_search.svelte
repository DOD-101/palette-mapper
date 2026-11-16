<script lang="ts">
  import ClearBtn from "$lib/clear_btn.svelte";
  import OptionList from "$lib/option_list.svelte";
  import * as wasm from "../../wasm/pkg/";
  import "iconify-icon";

  let { selected = $bindable() }: { selected: string } = $props();

  let selected_pretty = $derived.by(() => {
    if (selected.match(/^base/)) {
      const palette = selected.slice(8);
      const base = palette.slice(4, 6);
      const name = palette.slice(6);

      return `${name} (Base${base})`;
    }

    return "";
  });

  let searched = $state("");

  type PaletteVariant = {
    value: string;
    badge: string;
    colors: string[];
  };

  type PaletteOption = {
    value: string;
    label: string;
    variants: PaletteVariant[];
  };

  await wasm.default();

  const palette_groups = new Map<string, PaletteOption>();

  for (const palette of wasm.base_both()) {
    const variant: PaletteVariant = {
      value: `base16::${palette}`,
      badge: `Base${palette.slice(4, 6)}`,
      colors: JSON.parse(
        wasm.pal_from_base(wasm.from_base_name(palette)),
      ) as string[],
    };

    const label = palette.slice(6);
    const group = palette_groups.get(label);

    if (group) {
      group.variants.push(variant);
    } else {
      palette_groups.set(label, {
        value: variant.value,
        label,
        variants: [variant],
      });
    }
  }

  const palettes = [...palette_groups.values()];

  for (const palette of palettes) {
    palette.variants.sort((a, b) => a.badge.localeCompare(b.badge));
  }

  function search_theme(palette: PaletteOption) {
    // smart-case
    const term = `${palette.label} ${palette.variants
      .map((variant) => variant.badge)
      .join(" ")}`;

    if (searched.toLowerCase() == searched) {
      return term.toLowerCase().includes(searched);
    }

    return term.includes(searched);
  }

  let palette_options = $derived.by(() => palettes.filter(search_theme));
</script>

<div class="search-container">
  <div class="search-box input-elm">
    {#if selected}
      <ClearBtn
        onclick={() => {
          selected = "";
          searched = "";
        }}
        size={1}
        classNames="palette-search-clear"
      />
    {:else}
      <iconify-icon icon="material-symbols:search-rounded"> </iconify-icon>
    {/if}

    <input
      type="search"
      bind:value={searched}
      placeholder={selected ? selected_pretty : "Search palette"}
      aria-label="Search palette"
    />
  </div>

  <OptionList
    options={palette_options}
    bind:value={selected}
    label="Palette"
    maxHeight="45vh"
  />
</div>

<style>
  .search-container {
    width: 100%;
  }

  .search-box {
    display: flex;
    align-items: center;
    position: relative;
    width: 100%;
    margin-bottom: 0.35rem;
    padding: 0 0.25rem;
    box-sizing: border-box;
    background-color: var(--secondary);
    border-radius: 6px;

    input {
      width: 100%;
      margin-left: 1.1rem;
      padding: 0.1rem 0;
      border: none;
      background-color: transparent;
      color: var(--text);
      font: inherit;
      box-sizing: border-box;
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
</style>

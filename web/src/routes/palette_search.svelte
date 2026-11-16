<script lang="ts">
  import OptionList from "$lib/option_list.svelte";
  import type { Option, Variant } from "$lib/option";
  import * as wasm from "../../wasm/pkg/";
  import "iconify-icon";

  let { selected = $bindable() }: { selected: string } = $props();

  let searched = $state("");

  type PaletteGroup = Option & { variants: Variant[] };

  await wasm.default();

  /**
   * Split a `BaseBoth` display name (`Base16<name>` / `Base24<name>`) into its
   * badge and human-readable label.
   */
  function describe_palette(palette: string): { badge: string; label: string } {
    return {
      badge: `Base${palette.slice(4, 6)}`,
      label: palette.slice(6),
    };
  }

  function palette_colors(palette: string): string[] {
    const base = wasm.from_base_name(palette);

    return JSON.parse(wasm.pal_from_base(base)) as string[];
  }

  const palette_groups = new Map<string, PaletteGroup>();

  for (const palette of wasm.base_both()) {
    const { badge, label } = describe_palette(palette);
    const variant: Variant = {
      value: palette,
      badge,
      colors: palette_colors(palette),
    };

    const group = palette_groups.get(label);

    if (group) {
      group.variants.push(variant);
    } else {
      palette_groups.set(label, {
        value: palette,
        label,
        variants: [variant],
      });
    }
  }

  const palettes = [...palette_groups.values()];

  for (const palette of palettes) {
    palette.variants.sort((a, b) => a.badge.localeCompare(b.badge));
    palette.value = palette.variants[0].value;
  }

  function search_theme(palette: PaletteGroup): boolean {
    const term = `${palette.label} ${palette.variants
      .map((variant) => variant.badge)
      .join(" ")}`;

    // smart-case: lowercase queries match case-insensitively
    return searched.toLowerCase() === searched
      ? term.toLowerCase().includes(searched)
      : term.includes(searched);
  }

  const palette_options = $derived(palettes.filter(search_theme));
</script>

<div class="search-container">
  <div class="search-box">
    <iconify-icon icon="material-symbols:search-rounded"></iconify-icon>

    <input
      type="search"
      bind:value={searched}
      placeholder="Search palettes"
      aria-label="Search palettes"
    />
  </div>

  <div class="search-divider" aria-hidden="true"></div>

  <OptionList
    options={palette_options}
    bind:value={selected}
    label="Palette"
    maxHeight="none"
  />
</div>

<style>
  .search-container {
    display: flex;
    flex: 1;
    flex-direction: column;
    min-height: 0;
    width: 100%;
  }

  .search-box {
    display: flex;
    flex: none;
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;
    position: relative;
    width: 100%;
    padding: 0.6rem 0.7rem;
    box-sizing: border-box;
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    background-color: var(--surface-control);
    transition:
      border-color 150ms ease-in-out,
      background-color 150ms ease-in-out;

    &:focus-within {
      border-color: var(--accent);
      background-color: var(--surface-control-hover);
    }

    input {
      flex: 1;
      min-width: 0;
      margin: 0;
      padding: 0;
      appearance: none;
      border: none;
      background-color: transparent;
      color: var(--text);
      font: inherit;
      box-sizing: border-box;

      &::placeholder {
        color: var(--text-muted);
      }

      &:focus {
        outline: none;
        box-shadow: none;
      }

      &::-webkit-search-cancel-button {
        display: none;
      }
    }

    iconify-icon {
      flex: none;
      color: var(--accent);
    }
  }

  .search-divider {
    flex: none;
    width: 100%;
    height: 1px;
    margin: 0.55rem 0;
    background-color: var(--border-subtle);
  }

  .search-container :global(.option-list) {
    flex: 1;
    min-height: 5rem;
  }
</style>

<script lang="ts">
  import type { Option } from "./option.ts";

  let {
    option,
    value = $bindable(),
  }: {
    option: Option;
    value: string;
  } = $props();

  const variants = $derived(option.variants ?? []);
  const active = $derived(
    variants.find((variant) => variant.value === value) ?? variants[0],
  );
  const active_index = $derived(
    variants.findIndex((variant) => variant.value === active?.value),
  );
  const next = $derived(
    variants.length > 1
      ? variants[(active_index + 1) % variants.length]
      : undefined,
  );
  const badge = $derived(active?.badge ?? option.badge);
  const colors = $derived(active?.colors ?? option.colors);
  const is_selected = $derived(
    variants.length > 0
      ? variants.some((variant) => variant.value === value)
      : option.value === value,
  );

  function select() {
    value = active?.value ?? option.value;
  }
</script>

<div class="option" class:selected={is_selected} class:has-variants={next}>
  <button
    type="button"
    class="option-select"
    aria-pressed={is_selected}
    title={option.label}
    onclick={select}
  >
    <span class="option-head">
      <span class="option-label">{option.label}</span>

      {#if !next && badge}
        <span class="option-badge">{badge}</span>
      {/if}
    </span>

    {#if colors?.length}
      <span class="swatches" aria-hidden="true">
        {#each colors as color}
          <span style:background-color={color}></span>
        {/each}
      </span>
    {/if}
  </button>

  {#if next}
    <button
      type="button"
      class="option-badge toggle"
      style:--active-index={active_index}
      title={`Switch to ${next.badge} version`}
      aria-label={`Switch to ${next.badge} version`}
      onclick={() => (value = next.value)}
    >
      {#each variants as variant (variant.value)}
        <span
          class="toggle-option"
          class:active={variant.value === active?.value}>{variant.badge}</span
        >
      {/each}
    </button>
  {/if}
</div>

<style>
  .option {
    position: relative;
    display: block;
    width: 100%;
    box-sizing: border-box;
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    margin: 1px 0;
    color: var(--text);
    background-color: transparent;
    transition: background-color 150ms ease-in-out;

    &:hover {
      background-color: var(--surface-hover);
    }

    &.selected {
      background-color: var(--surface-selected);
      outline: 2px solid var(--accent);
      outline-offset: -2px;
    }
  }

  .option-select {
    display: flex;
    width: 100%;
    min-width: 0;
    box-sizing: border-box;
    flex-direction: column;
    gap: 0.5rem;
    padding: 0.7rem;
    border: none;
    border-radius: inherit;
    background: none;
    color: inherit;
    font: inherit;
    text-align: left;
    cursor: pointer;

    &:focus-visible {
      outline: 2px solid var(--accent);
      outline-offset: -2px;
    }
  }

  .option.selected .option-select {
    font-weight: 600;
  }

  .option-head {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 0.5rem;
    width: 100%;
  }

  /* Keep the label clear of the absolutely positioned variant toggle. */
  .option.has-variants .option-head {
    padding-right: 8rem;
    box-sizing: border-box;
  }

  .option-label {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .option-badge {
    flex: none;
    padding: 0.1rem 0.45rem;
    border: 1px solid var(--border-subtle);
    border-radius: 999px;
    background-color: var(--surface);
    color: var(--text);
    font: inherit;
    font-size: 0.7rem;
    font-weight: 600;
    line-height: 1.2;
    white-space: nowrap;
  }

  .option-badge.toggle {
    position: absolute;
    top: 0.7rem;
    right: 0.7rem;
    display: grid;
    grid-auto-flow: column;
    grid-auto-columns: 1fr;
    width: 7rem;
    margin: 0;
    padding: 2px;
    overflow: hidden;
    cursor: pointer;
    box-sizing: border-box;
    transition: scale 150ms ease-in-out;
  }

  .option-badge.toggle::before {
    content: "";
    position: absolute;
    top: 2px;
    bottom: 2px;
    left: 2px;
    width: calc(50% - 2px);
    border-radius: 999px;
    background-color: var(--accent);
    transform: translateX(calc(var(--active-index, 0) * 100%));
    transition: transform 180ms ease-in-out;
  }

  .option-badge.toggle:hover {
    scale: 1.06;
  }

  .toggle-option {
    position: relative;
    z-index: 1;
    padding: 0.1rem 0.35rem;
    font-size: 0.7rem;
    font-weight: 600;
    line-height: 1.2;
    text-align: center;
    opacity: 0.6;
    white-space: nowrap;
    transition: opacity 180ms ease-in-out;
  }

  .toggle-option.active {
    color: var(--accent-contrast);
    opacity: 1;
  }

  .swatches {
    display: flex;
    gap: 1px;
    width: 100%;
    height: 0.8rem;
    overflow: hidden;
    border-radius: 2px;
  }

  .swatches span {
    flex: 1;
    min-width: 0;
  }
</style>

<script lang="ts">
  type Variant = {
    value: string;
    badge: string;
    colors: string[];
  };

  type Option = {
    value: string;
    label: string;
    badge?: string;
    colors?: string[];
    variants?: Variant[];
  };

  let {
    options,
    value = $bindable(),
    label,
    maxHeight = "50rem",
  }: {
    options: Option[];
    value: string;
    label?: string;
    maxHeight?: string;
  } = $props();
</script>

<div
  class="option-list"
  role="listbox"
  aria-label={label}
  style:max-height={maxHeight}
>
  {#each options as option (option.value)}
    {@const variants = option.variants ?? []}
    {@const active =
      variants.find((variant) => variant.value === value) ?? variants[0]}
    {@const active_index = variants.findIndex(
      (variant) => variant.value === active?.value,
    )}
    {@const next =
      variants.length > 1
        ? variants[(active_index + 1) % variants.length]
        : undefined}
    {@const badge = active?.badge ?? option.badge}
    {@const colors = active?.colors ?? option.colors}
    {@const is_selected =
      variants.length > 0
        ? variants.some((variant) => variant.value === value)
        : option.value === value}

    <div
      class="option"
      class:selected={is_selected}
      role="option"
      aria-selected={is_selected}
      tabindex="0"
      title={option.label}
      onclick={() => (value = active?.value ?? option.value)}
      onkeydown={(event) => {
        if (event.target !== event.currentTarget) {
          return;
        }

        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          value = active?.value ?? option.value;
        }
      }}
    >
      <span class="option-head">
        <span class="option-label">{option.label}</span>

        {#if next}
          <button
            type="button"
            class="option-badge toggle"
            style:--active-index={active_index}
            title={`Switch to ${next.badge} version`}
            aria-label={`Switch to ${next.badge} version`}
            onclick={(event) => {
              event.stopPropagation();
              value = next.value;
            }}
            onkeydown={(event) => event.stopPropagation()}
          >
            {#each variants as variant}
              <span
                class="toggle-option"
                class:active={variant.value === active?.value}
                >{variant.badge}</span
              >
            {/each}
          </button>
        {:else if badge}
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
    </div>
  {:else}
    <p class="empty">No matches</p>
  {/each}
</div>

<style>
  .option-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
    width: 100%;
    padding: 2px;
    overflow-y: auto;
    overscroll-behavior: contain;
    box-sizing: border-box;
    background-color: var(--primary);
    border: 2px solid var(--grid-lines);
    border-radius: 4px;
    scrollbar-color: var(--accent) transparent;
  }

  .option {
    display: flex;
    flex: none;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
    padding: 0.55rem;
    overflow: hidden;
    box-sizing: border-box;
    border-radius: 3px;
    background-color: var(--secondary);
    color: var(--text);
    text-align: left;
    cursor: pointer;
    transition: background-color 150ms ease-in-out;
  }

  .option-head {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 0.5rem;
    width: 100%;
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
    border: 1px solid var(--grid-lines);
    border-radius: 999px;
    background-color: var(--background);
    color: var(--text);
    font: inherit;
    font-size: 0.7rem;
    font-weight: 600;
    line-height: 1.2;
    white-space: nowrap;
  }

  .option-badge.toggle {
    position: relative;
    display: grid;
    grid-auto-flow: column;
    grid-auto-columns: 1fr;
    padding: 2px;
    overflow: hidden;
    cursor: pointer;
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
    background-color: var(--confirm);
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
    opacity: 0.6;
    white-space: nowrap;
    transition: opacity 180ms ease-in-out;
  }

  .toggle-option.active {
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

  .option:hover {
    background-color: var(--accent);
  }

  .option.selected {
    background-color: var(--accent);
    font-weight: 600;
    outline: 2px solid var(--confirm);
    outline-offset: -2px;
  }

  .option:focus-visible {
    outline: 2px solid var(--confirm);
    outline-offset: 2px;
  }

  .empty {
    margin: 0;
    padding: 0.35rem;
    font-size: 0.85rem;
    text-align: center;
    opacity: 0.7;
  }
</style>

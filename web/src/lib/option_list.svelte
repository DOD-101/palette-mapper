<script lang="ts">
  import OptionCard from "./option_card.svelte";
  import type { Option } from "./option.ts";

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

<ul class="option-list" aria-label={label} style:max-height={maxHeight}>
  {#each options as option (option.value)}
    <li class="option-list-item">
      <OptionCard {option} bind:value />
    </li>
  {:else}
    <li class="empty">No matches</li>
  {/each}
</ul>

<style>
  .option-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
    width: 100%;
    margin: 0;
    padding: 2px;
    list-style: none;
    overflow-y: auto;
    overscroll-behavior: contain;
    box-sizing: border-box;
    border: 2px solid var(--border);
    border-radius: 4px;
    scrollbar-color: var(--accent) transparent;
  }

  .option-list-item {
    display: flex;
    width: 100%;
  }

  .empty {
    margin: 0;
    padding: 0.35rem;
    font-size: 0.85rem;
    text-align: center;
    opacity: 0.7;
  }
</style>

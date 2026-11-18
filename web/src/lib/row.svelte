<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    children,
    classNames = "",
  }: { children?: Snippet; classNames?: string } = $props();
</script>

<div class={["row", classNames]}>
  {@render children?.()}
</div>

<style>
  .row {
    padding: 3px 0;
  }
</style>

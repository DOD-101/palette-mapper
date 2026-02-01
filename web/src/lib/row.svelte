<script lang="ts">
    let { children }: { children: any } = $props();
</script>

<div class="row">
    {@render children?.()}
</div>

<style>
    .row {
        padding: 5px 0;
    }
</style>

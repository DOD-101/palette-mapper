<script lang="ts">
    import { notifications } from "./notifications.svelte.ts";
    import UiNotification from "./ui_notification.svelte";
</script>

<div class="notifications">
    {#each notifications.data as n, i (n)}
        <UiNotification data={n} pos={i} />
    {/each}
</div>

<style>
    .notifications {
        position: absolute;
        top: 0;
        right: 0;
        width: 400px;
        z-index: 100;
    }
</style>

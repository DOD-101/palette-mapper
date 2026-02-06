<script lang="ts">
    import { onMount } from "svelte";
    import type { UiNotification } from "./notifications.svelte.ts";
    import { notificationConfig } from "./notifications.svelte.ts";

    let { data, pos }: { data: UiNotification; pos: number } = $props();

    let opacity = $state(1);

    onMount(() => {
        setTimeout(
            () => {
                opacity = 0;
            },
            notificationConfig.time - notificationConfig.time / 10,
        );
    });
</script>

<div
    class={["notification", data.level]}
    style:top="{103 * pos}px"
    style:opacity
>
    <div class="content">
        <p class="level">{data.level}</p>
        <p class="text">{@html data.msg}</p>
    </div>
</div>

<style>
    .notification {
        animation: 500ms cubic-bezier(0.175, 0.885, 0.32, 1.275) 0s 1 normal
            forwards running slide-in;
        height: 100px;
        width: 100%;
        position: absolute;
        right: 5px;
        opacity: 1;
        transform: translateX(120%);

        transition:
            top 300ms linear,
            opacity 500ms ease-out;
        padding: 5px;
        box-sizing: border-box;
    }

    .content {
        border: 3px solid var(--grid-lines);
        background-image: radial-gradient(var(--dots) 1px, transparent 0);
        background-size: 16px 16px;
        height: 100%;
        display: flexbox;
        align-content: center;
        padding: 2px;
        padding-left: 10px;
        border-radius: 6px;

        color: var(--text);

        .level {
            margin-bottom: 0px;
            margin-top: 10px;
            font-weight: 700;
        }
    }

    @keyframes slide-in {
        to {
            transform: translateX(0);
        }
    }

    .ERROR .content {
        background-color: var(--error);
    }

    .WARN .content {
        background-color: var(--warn);
    }

    .INFO .content {
        background-color: var(--info);
    }
</style>

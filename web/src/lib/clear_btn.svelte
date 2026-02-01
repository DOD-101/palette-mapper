<script lang="ts">
    const {
        classNames,
        onclick,
        hidden,
        size = 1.5,
    }: {
        classNames?: string;
        onclick: () => any;
        hidden?: boolean;
        size?: number;
    } = $props();

    import "iconify-icon";
</script>

<button
    {onclick}
    class={classNames}
    aria-label="Clear"
    {hidden}
    style:height="{size}rem"
    style:width="{size}rem"
>
    <iconify-icon
        icon="material-symbols:close-rounded"
        height="1rem"
        width="1rem"
    >
    </iconify-icon>
</button>

<style>
    button {
        position: relative;
        aspect-ratio: 1;
        border-radius: 10rem;
        border: none;
        background-color: #ff0047;

        iconify-icon {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
        }
    }
</style>

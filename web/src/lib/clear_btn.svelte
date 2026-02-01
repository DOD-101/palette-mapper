<script lang="ts">
    const {
        classNames,
        onclick,
        hidden,
    }: { classNames?: string; onclick: () => any; hidden?: boolean } = $props();

    import "iconify-icon";
</script>

<button {onclick} class={classNames} aria-label="Clear" {hidden}>
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
        height: 1.5rem;
        width: 1.5rem;
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

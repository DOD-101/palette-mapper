<script lang="ts">
    let {
        label,
        files = $bindable(),
    }: { label: string; files: FileList | undefined } = $props();
</script>

<label for="file_upload" id="file_upload_label">{label}</label>
<input type="file" id="file_upload" bind:files required />

<style>
    label {
        display: inline-block;
        color: var(--text);
        text-align: center;
        background-color: var(--secondary);
        cursor: pointer;
        border-radius: 3px;
        padding: 0.1rem 0;
        margin: 0.25rem 0.5rem;

        transition: all 200ms ease-in-out;

        &:hover {
            background-color: var(--accent);
            scale: 1.02;
        }
    }

    input[type="file"] {
        display: none;
    }
</style>

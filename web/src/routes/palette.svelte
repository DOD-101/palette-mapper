<script lang="ts">
    let { palette = $bindable() }: { palette: string } = $props();

    let pal_files: FileList | undefined = $state();
    let pal_label: string = $derived.by(() => {
        if (pal_files === undefined) {
            return "Palette";
        }

        let file = pal_files.item(0);

        if (file === null) {
            return "Palette";
        }

        return file.name;
    });

    // TODO: Add selection of pal from palettes/
    let pal_upload = true;

    /** Read inputted palette from the given file */
    $effect(() => {
        let file = pal_files?.item(0);
        if (file) {
            const palette_reader = new FileReader();

            palette_reader.onload = (reader_ev) => {
                if (reader_ev.target == null) {
                    return;
                }

                const contents = reader_ev.target.result;

                if (typeof contents !== "string") {
                    return;
                }

                palette = contents;

                console.log(palette);
            };

            palette_reader.readAsText(file);
        }
    });
</script>

<div class="row">
    <label for="pal_upload" id="pal_upload_label">{pal_label}</label>
    <input
        type="file"
        id="pal_upload"
        name="palette"
        bind:files={pal_files}
        hidden={!pal_upload}
        required
    />
</div>

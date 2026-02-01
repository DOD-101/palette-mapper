<script lang="ts">
    import Sidebar from "./sidebar.svelte";
    import { converted_img } from "./sidebar.svelte.ts";

    let img: string | undefined = $state();

    $effect(() => {
        if (converted_img.data) {
            img = URL.createObjectURL(converted_img.data);
        }
    });
</script>

<h1>Palette Mapper</h1>

<Sidebar />

<div id="img_container">
    <img alt="" id="img_preview" src={img} />
</div>

<style>
    h1 {
        position: absolute;
        left: 0;
        top: 0;

        margin: 2rem;
    }

    #img_container {
        position: relative;
        display: block;
        width: 100%;

        #img_preview {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            max-width: 85vw;
            max-height: 95vh;
            width: auto;
            height: auto;
        }
    }
</style>

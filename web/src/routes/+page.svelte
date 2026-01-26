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

<Sidebar />

<div id="img_container">
    <img alt="" id="img_preview" src={img} />
</div>

<style>
    #img_container {
        display: block;
        position: relative;
        #img_preview {
            position: absolute;
            top: 50%;
            left: 0;
            transform: translateY(-50%);
            max-width: 85vw;
            max-height: 95vh;
            width: auto;
            height: auto;
        }
    }
</style>

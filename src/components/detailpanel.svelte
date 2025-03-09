<script lang="ts">
    let { file } = $props();
</script>

<div class="w-full h-full ml-4">
    <div class="w-fit px-2 text-md">Details</div>
    {#if file.length !== 0}
    <div class="w-full h-full py-2 flex flex-row">
        <div class=" h-4/5 aspect-square flex items-center justify-center shadow-lg">
            <img class="rounded" src="https://placehold.co/400x400?text=Thumbnail" alt="" />
        </div>
        <div class="w-2/6 ml-12">
            <div class=" ">Filepath: <span class="font-bold">{file[0].name.replace(/\.[^/.]+$/, "")}</span></div>
            <div class=" ">File Format: <span class="font-bold">{file[0].type.split("/").pop()}</span></div>
            <div class=" ">Duration:</div>
            <div class=" ">Size:</div>
        </div>
        <div class="ml-12">
            <div class=" ">Dimentions:</div>
            <div class=" ">Frame Rate:</div>
            <div class=" ">Bit Rate:</div>
            <div class=" ">Aspect Ratio:</div>
        </div>
    </div>
    {/if}
</div>
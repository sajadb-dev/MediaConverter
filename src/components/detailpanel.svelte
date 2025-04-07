<script lang="ts">
    let { 
            file, 
            filepath, 
            duration, 
            format, 
            size, 
            width,
            height,
            framerate,
            bitrate,
            aspectratio,
            thumbnail } = $props();

    let dimention = $derived(`${width}x${height}`);

</script>

<div class="w-full h-full ml-4">
    <div class="w-fit px-2 text-sm">Details</div>
    {#if file}
    <div class="w-full h-full flex text-sm">
        <div class=" h-4/5 aspect-square flex items-center justify-center shadow-lg">
            <img class="w-full h-full object-cover rounded" src={thumbnail} alt="" />
        </div>
        <div class="w-full h-9/12 mx-12 grid grid-cols-3 gap-1">
            <div class="flex flex-col"><span>Filepath:</span><span class=" bg-gray-100 rounded-md p-1 font-bold truncate ml-2">{filepath}</span></div>
            <div class="flex flex-col"><span>File Format:</span><span class="bg-gray-100 rounded-md p-1 font-bold truncate ml-2">{format}</span></div>
            <div class="flex flex-col"><span>Duration:</span><span class="bg-gray-100 rounded-md p-1 font-bold truncate ml-2">{duration}</span></div>
            <div class="flex flex-col"><span>Size:</span><span class="bg-gray-100 rounded-md p-1 font-bold truncate ml-2">{size} MB</span></div>
            <div class="flex flex-col"><span>Dimentions:</span><span class="bg-gray-100 rounded-md p-1 font-bold truncate ml-2">{dimention}</span></div>
            <div class="flex flex-col"><span>Frame Rate:</span><span class="bg-gray-100 rounded-md p-1 font-bold truncate ml-2">{framerate}</span></div>
            <div class="flex flex-col"><span>Bit Rate:</span><span class="bg-gray-100 rounded-md p-1 font-bold truncate ml-2">{bitrate}</span></div>
            <div class="flex flex-col"><span>Aspect Ratio:</span><span class="bg-gray-100 rounded-md p-1 font-bold truncate ml-2">{aspectratio}</span></div>
        </div>
    </div>
    {/if}
</div>
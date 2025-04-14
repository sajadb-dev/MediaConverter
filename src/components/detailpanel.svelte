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
            aspectratio } = $props();

    let dimention = $derived(`${width}x${height}`);

    function formatFileSize(bytes: number) {
        const KB = 1024;
        const MB = KB * 1024;
        const GB = MB * 1024;

        if (bytes < KB) {
          return `${bytes} B`;
        } else if (bytes < MB) {
          return `${(bytes / KB).toFixed(2)} KB`;
        } else if (bytes < GB) {
          return `${(bytes / MB).toFixed(2)} MB`;
        } else {
          return `${(bytes / GB).toFixed(2)} GB`;
        }
    }
</script>

<div class="w-full h-full">
    <div class="w-fit p-2 text-sm">Details:</div>
    {#if file}
    <div class="w-full h-full flex flex-col gap-1 mt-2 px-2">
        <label for="videotitle" class="text-xs font-bold">File Path:</label>
        <input id="videotitle" value={filepath} class="w-full h-6 px-2 rounded bg-gray-100" type="text" disabled/>
        <label for="videotitle" class="text-xs font-bold">Format:</label>
        <input id="videotitle" value={format} class="w-full h-6 px-2 rounded bg-gray-100" type="text" disabled/>
        <label for="videotitle" class="text-xs font-bold">File Size:</label>
        <input id="videotitle" value={formatFileSize(size)} class="w-full h-6 px-2 rounded bg-gray-100" type="text" disabled/>
        <label for="videotitle" class="text-xs font-bold">Dimention:</label>
        <input id="videotitle" value={dimention} class="w-full h-6 px-2 rounded bg-gray-100" type="text" disabled/>
        <label for="videotitle" class="text-xs font-bold">Duration:</label>
        <input id="videotitle" value={duration} class="w-full h-6 px-2 rounded bg-gray-100" type="text" disabled/>
        <label for="videotitle" class="text-xs font-bold">Framerate:</label>
        <input id="videotitle" value={framerate} class="w-full h-6 px-2 rounded bg-gray-100" type="text" disabled/>
        <label for="videotitle" class="text-xs font-bold">Bitrate:</label>
        <input id="videotitle" value={bitrate} class="w-full h-6 px-2 rounded bg-gray-100" type="text" disabled/>
        <label for="videotitle" class="text-xs font-bold">Aspect Ratio:</label>
        <input id="videotitle" value={aspectratio} class="w-full h-6 px-2 rounded bg-gray-100" type="text" disabled/>
    </div>
    {/if}
</div>
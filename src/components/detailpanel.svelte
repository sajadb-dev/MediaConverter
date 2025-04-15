<script lang="ts">
  import Metadataitem from "./metadataitem.svelte";

    let { file, metadata } = $props();

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

<div class="w-full h-full bg-[var(--button-secondary)]/60 overflow-x-scroll">
    <div class="w-fit p-2 text-sm">Details:</div>
    {#if file}
    <div class="w-full h-full flex flex-col gap-2 my-2 px-2 ">
      <div class="flex flex-col gap-2">
        <Metadataitem label="File Name" inputvalue={metadata.file_name}/>
        <Metadataitem label="File path" inputvalue={metadata.file_path}/>
        <Metadataitem label="File path" inputvalue={formatFileSize(metadata.file_size)}/>
      </div>
      {#each metadata.streams as stream, i}
      <div class="flex flex-col gap-1 bg-[var(--background)]/50 rounded-md p-1">
        <h3 class="font-bold">Stream {i} ({stream.medium}) :</h3>
        <Metadataitem label="Codec" inputvalue={stream.codec_id}/>
        <Metadataitem label="Duration" inputvalue={stream.duration_seconds?.toFixed(2)}/>
        <Metadataitem label="Frames" inputvalue={stream.frames}/>
        {#if stream.video}
          <Metadataitem label="Resolution" inputvalue={`${stream.video.width}x${stream.video.height}`}/>
          <Metadataitem label="Bitrate" inputvalue={stream.video.bit_rate}/>
          <Metadataitem label="Format" inputvalue={stream.video.format}/>
        {/if}
        {#if stream.audio}
          <Metadataitem label="Channels" inputvalue={stream.audio.channels}/>
          <Metadataitem label="Rate" inputvalue={`${stream.audio.rate} Hz`}/>
          <Metadataitem label="Layout" inputvalue={stream.audio.channel_layout}/>
        {/if}
      </div>
    {/each}
    </div>
    {/if}
</div>
<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { Progress } from "melt/builders";
    import { onMount } from "svelte";
    import Skeleton from "./skeleton/skeleton.svelte";
    
    const { classnames , title, progressvalue, focus, filepath, index } = $props()
    let thumbnail = $state('');
    const progress = new Progress();

    $effect(()=> {
        if(progressvalue !== 0.00)
        progress.value = progressvalue;
    })

    async function loadThumbnail(path: string) {
        const imageBytes: number[] = await invoke('get_thumbnail_image', { path });
        const blob = new Blob([new Uint8Array(imageBytes)], { type: 'image/jpeg' });
        return URL.createObjectURL(blob);
    }

 onMount( async () => {
    let temp: string = await invoke('generate_thumbnail' ,{path: filepath});
    thumbnail = await loadThumbnail(temp);
 });
</script>
<div class="{classnames} w-[95%] min-h-22 max-h-22 flex bg-[var(--file-item-background)] rounded-sm overflow-hidden focus:ring-2" onclick={focus} onkeydown={focus} role="button" tabindex={index}>
    <div class="h-full min-w-6 flex items-center justify-center border-r border-[var(--outline)]">
        <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-width="1.5" d="M20 7H4m16 5H4m16 5H4"/></svg>
    </div>
    <div class="w-full flex justify-between items-center pl-2 pr-4 gap-2">
        <div class="h-full flex items-center gap-4">
            <div class=" h-4/5 aspect-square flex items-center justify-center shadow-lg">
                {#if thumbnail === ""}
                <Skeleton classnames="h-full w-full" />
                {:else}
                <img class="w-full h-full object-cover rounded" src={thumbnail} alt="" />
                {/if}
            </div>
            <div class="w-52 flex flex-col text-sm truncate">
                <label for="" class="font-bold">Title:</label>
                <p class="whitespace-nowrap">
                    {title}
                </p>
            </div>
        </div>
        <div class="w-48 py-3 flex flex-col items-center text-sm">
            <label for="" class="font-bold">Progress:</label>
            <div {...progress.root} class="w-full min-w-20 h-4 bg-white rounded-sm overflow-hidden">
                <div {...progress.progress} class="h-full {progress.value === 100 ? 'bg-green-300' : 'bg-amber-300'}"></div>
            </div>
        </div>
    </div>
</div>

<style>
  [data-melt-progress-progress] {
    transform: translateX(calc(var(--progress) * -1));
  }
</style>
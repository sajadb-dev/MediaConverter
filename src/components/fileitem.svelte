<script lang="ts">
    import { Progress } from "melt/builders";
    
    const { title, progressvalue = 0, focus, index} = $props()
    
    const progress = new Progress();
    progress.value = progressvalue;

    if(progress.value === 100) {

     }
 
</script>
<div class="w-[90%] min-h-22 max-h-22 flex bg-slate-100 rounded-sm overflow-hidden focus:ring-2" onclick={focus} onkeydown={focus} role="button" tabindex={index}>
    <div class="h-full min-w-6 flex items-center justify-center border-r border-slate-200">
        <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-width="1.5" d="M20 7H4m16 5H4m16 5H4"/></svg>
    </div>
    <div class="w-full flex justify-between items-center pl-2 pr-4 gap-2">
        <div class="h-full flex items-center gap-4">
            <div class=" h-4/5 aspect-square flex items-center justify-center shadow-lg">
                <img class="rounded" src="https://placehold.co/400x400?text=Thumbnail" alt="" />
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
            <div {...progress.root} class="w-full h-4 bg-white rounded-sm overflow-hidden">
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
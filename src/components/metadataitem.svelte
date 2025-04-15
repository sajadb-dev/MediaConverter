<script>
    const { label, inputvalue} = $props();
</script>

<div class="w-full flex flex-col relative">
    <label for="videotitle" class="text-xs font-bold relative">{label}</label>
    <div class="w-2 h-4 border-b border-l border-[var(--active-color)] rounded-bl-md absolute left-2 bottom-2"></div>
    <input id="videotitle" value={inputvalue} class="w- h-6 ml-4 px-2 rounded bg-[var(--input-background)]" type="text" disabled/>
</div>
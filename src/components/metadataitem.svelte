<script>
    const { label, inputvalue} = $props();
</script>

<label for="videotitle" class="text-xs font-bold">{label}</label>
<input id="videotitle" value={inputvalue} class="w-full h-6 px-2 rounded bg-[var(--input-background)]" type="text" disabled/>
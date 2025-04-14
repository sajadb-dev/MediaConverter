<script lang="ts">

    const codecList = [
        { value: 'h.264', label: 'H.264' },
        { value: 'h.265', label: 'H.265/HEVC' },
        { value: 'mpeg-1', label: 'MPEG-1' },
        { value: 'mpeg-2', label: 'MPEG-2' },
        { value: 'mpeg-4', label: 'MPEG-4' },
        { value: 'av1', label: 'AV1' },
        { value: 'webm', label: 'WebM/VP9' },
        { value: 'theora', label: 'Theora' }
    ];
    const containerList = [
        { value: 'matroska', label: 'Matroska' },
        { value: 'webm', label: 'WebM' },
        { value: 'mpeg-1', label: 'MPEG-1' },
        { value: 'mpeg-2', label: 'MPEG-2' },
        { value: 'mpeg-4', label: 'MPEG-4' },
        { value: 'ogg', label: 'OGG' },
        { value: 'avi', label: 'AVI' }
    ];

    const encodingSpeed = [
        { value: '1', label: 'Slowest' },
        { value: '2', label: 'Good' },
        { value: '3', label: 'Realtime' },
    ]
</script>

<div class="w-full h-full bg-[var(--button-secondary)]/60">
    <div class="w-fit p-2 text-sm">Output Setting:</div>
    <div class="w-full h-full mt-2 px-2">
        <label for="videotitle" class="text-xs font-bold">Output Title:</label>
        <input id="videotitle" class="w-full h-6 px-2 rounded bg-[var(--input-background)]" type="text"/>
        <label for="containers" class="text-xs font-bold">Output Container:</label>
        <select name="containers" class="w-full h-6 px-2 rounded text-sm bg-[var(--select-color)] hover:bg-[var(--select-color)]/50">
            {#each containerList as item }
            <option value={item.value}>{item.label}</option>
            {/each}
        </select>
        <label for="codecs" class="text-xs font-bold">Output Codec:</label>
        <select name="codecs" class="w-full h-6 px-2 rounded text-sm bg-[var(--select-color)] hover:bg-[var(--select-color)]/50">
            {#each codecList as item }
            <option value={item.value}>{item.label}</option>
            {/each}
        </select>
        <label for="encodingspeed" class="text-xs font-bold">Encoding Speed:</label>
        <select name="encodingspeed" class="w-full h-6 px-2 rounded text-sm bg-[var(--select-color)] hover:bg-[var(--select-color)]/50">
            {#each encodingSpeed as item }
            <option value={item.value}>{item.label}</option>
            {/each}
        </select>
    </div>
</div>
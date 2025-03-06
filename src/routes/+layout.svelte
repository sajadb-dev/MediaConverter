<script lang="ts">
import '../app.css';
  import Titlebar from '../components/titlebar.svelte';

let { children } = $props();
</script>

<div class="w-full h-full flex flex-col justify-between">
    <div class="layer-1">
        <Titlebar/>
    </div>
</div>
{@render children()}
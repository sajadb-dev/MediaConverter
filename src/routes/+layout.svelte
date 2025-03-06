<script lang="ts">
import '../app.css';
  import Controlmenu from '../components/controlmenu.svelte';
  import Menubar from '../components/menubar.svelte';
  import Titlebar from '../components/titlebar.svelte';

let { children } = $props();
</script>

<div class="w-full h-full flex flex-col">
    <div class="layer-1">
        <Titlebar/>
        <Menubar/>
        <Controlmenu/>
    </div>
</div>
{@render children()}
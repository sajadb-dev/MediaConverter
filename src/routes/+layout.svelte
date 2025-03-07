<script lang="ts">
    import '../app.css';
    import Controlmenu from '../components/controlmenu.svelte';
    import Menubar from '../components/menubar.svelte';
    import Titlebar from '../components/titlebar.svelte';
    import '@fontsource/lobster';
    import '@fontsource-variable/nunito';

let { children } = $props();
</script>

<div class="flex flex-col">
    <div>
        <Titlebar/>
        <Menubar/>
        <Controlmenu/>
    </div>
</div>
{@render children()}
<script lang="ts">
    import { FileUpload } from "melt/builders";
    import { HSplitPane, VSplitPane } from 'svelte-split-pane';
    import Detailpanel from "../components/detailpanel.svelte";
    import Propertiespanel from "../components/propertiespanel.svelte";
    import Fileitem from "../components/fileitem.svelte";
    import { SvelteSet } from "svelte/reactivity";
    import Toolbar from '../components/toolbar.svelte';
    import Menubar from '../components/menubar.svelte';
    import Titlebar from '../components/titlebar.svelte';

    const fileUpload = new FileUpload({multiple : true, accept : "video/*"});

    const files = $derived.by(() => {
		if (fileUpload.selected instanceof SvelteSet) {
			return Array.from(fileUpload.selected);
		}
		return [fileUpload.selected].filter((f): f is File => !!f);
	});

    let selectedfile;

    function focuscap () {
        console.log("captured");
    }

    $effect(() => {console.log(files.length > 0)})
</script>
<div class="flex flex-col">
    <div>
        <Titlebar/>
        <Menubar addfile={fileUpload.trigger.onclick}/>
        <Toolbar addfile={fileUpload.trigger.onclick}/>
    </div>
</div>
<div class="h-full box-border overflow-hidden">
    <HSplitPane leftPaneSize="75%" rightPaneSize="25%" minLeftPaneSize="60%" minRightPaneSize="0px">
        <left slot="left">
            <VSplitPane topPanelSize="72%" downPanelSize="28%" minTopPaneSize="60%" minDownPaneSize="0px" >
                <top slot="top" class="h-full flex">
                    <div class="dot w-full h-full flex justify-center items-center">
                        <div class="w-full h-full justify-center items-center {files.length === 0 ? 'flex' : 'hidden'}">
                            <input {...fileUpload.input}/>
                            <div {...fileUpload.dropzone} class="relative w-9/12 min-h-9/12 flex justify-center items-center rounded-2xl border-2 border-dashed bg-slate-100 border-slate-300 cursor-pointer">
                              {#if fileUpload.isDragging}
                                Drop files here
                              {:else}
                                <span class="font-bold mr-1">Click to upload</span><p>or drag and drop</p>
                              {/if}
                            </div>
                        </div>
                        <div class="w-full h-full flex-col items-center py-4 gap-4 overflow-auto {files.length === 0 ? 'hidden' : 'flex'}">
                            {#each files as file, i (i)}
                                <Fileitem title={file.name.replace(/\.[^/.]+$/, "")} filetype={file.type} progressvalue={0} focus={focuscap} />
                            {/each}
                        </div>
                    </div>
                </top>
                <down slot="down">
                    <Detailpanel file={files}/>
                </down>
            </VSplitPane>
        </left>
        <right slot="right">
            <Propertiespanel/>
        </right>
    </HSplitPane>
</div>
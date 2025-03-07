<script lang="ts">
    import { FileUpload } from "melt/builders";
    import { HSplitPane, VSplitPane } from 'svelte-split-pane';
    import Detailpanel from "../components/detailpanel.svelte";
    import Propertiespanel from "../components/propertiespanel.svelte";
    import Fileitem from "../components/fileitem.svelte";

    const file = true;

    const fileUpload = new FileUpload();
</script>
<div class="h-full box-border overflow-hidden">
    <HSplitPane leftPaneSize="75%" rightPaneSize="25%" minLeftPaneSize="60%" minRightPaneSize="0px">
        <left slot="left">
            <VSplitPane topPanelSize="72%" downPanelSize="28%" minTopPaneSize="60%" minDownPaneSize="0px" >
                <top slot="top" class="h-full flex">
                    <div class="dot w-full h-full flex justify-center items-center">
                        {#if !file}
                        <input {...fileUpload.input} />
                        <div {...fileUpload.dropzone} class="relative w-9/12 min-h-9/12 flex justify-center items-center rounded-2xl border-2 border-dashed bg-slate-100 border-slate-300 cursor-pointer">
                          {#if fileUpload.isDragging}
                            Drop files here
                          {:else}
                            <span class="font-bold mr-1">Click to upload</span><p>or drag and drop</p>
                          {/if}
                        </div>
                        {:else}
                        <div class="w-full h-full flex flex-col items-center py-4 gap-4 overflow-auto">
                            <Fileitem title="" progressvalue = {2} />
                            <Fileitem title=""/>
                            <Fileitem title=""/>
                            <Fileitem title=""/>
                            <Fileitem title=""/>
                            <Fileitem title=""/>
                            <Fileitem title=""/>
                        </div>
                        {/if}
                    </div>
                </top>
                <down slot="down">
                    <Detailpanel/>
                </down>
            </VSplitPane>
        </left>
        <right slot="right">
            <Propertiespanel/>
        </right>
    </HSplitPane>
</div>
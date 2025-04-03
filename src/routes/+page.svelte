<script lang="ts">
    import { Pane, Splitpanes } from 'svelte-splitpanes';
    import Detailpanel from '../components/detailpanel.svelte';
    import Propertiespanel from '../components/propertiespanel.svelte';
    import Fileitem from '../components/fileitem.svelte';
    import Toolbar from '../components/toolbar.svelte';
    import Menubar from '../components/menubar.svelte';
    import Titlebar from '../components/titlebar.svelte';
    import { open } from '@tauri-apps/plugin-dialog';

    let isDragging = $state(false);
    let filepath: string[] = $state([]);
    let focusedfile: number = $state(0);

    function draghandle(){
        isDragging = !isDragging;
    }

    async function addfile(){
      const file = await open({multiple: true, directory: false,
        filters: [{ name: 'Video', extensions: ['mp4', 'm4a', 'avi'] }]
      });
      if(file) {
      file.forEach(element => {
        filepath.push(element);
      })};
      console.log(filepath)
    }

    function removefile() {
      filepath.splice(focusedfile, 1);
    }

    function focusgrab(index: number) {
      focusedfile = index;
      console.log(focusedfile);
    }

</script>

<div class="flex flex-col">
    <div>
        <Titlebar/>
        <Menubar addfile={addfile}/>
        <Toolbar addfile={addfile} removefile={removefile}/>
    </div>
</div>
<div class="h-full box-border overflow-hidden">
    <Splitpanes theme="my-theme">
        <Pane  size={75} minSize={20}>
            <Splitpanes theme="my-theme" horizontal={true}>
                <Pane size={75} minSize={60}>
                    <div class="dot w-full h-full flex justify-center items-center">
                      {#if filepath.length === 0}
                      <div class="w-full h-full flex justify-center items-center">
                            <button 
                                class="relative w-9/12 min-h-9/12 flex justify-center items-center rounded-2xl border-2 border-dashed bg-slate-100 border-slate-300 cursor-pointer"
                                ondragenter={draghandle}
                                ondragleave={draghandle}
                                onclick={addfile}
                                role="dialog"
                                tabindex="0">
                              {#if isDragging}
                                Drop files here
                              {:else}
                                <span class="font-bold mr-1">Click to upload</span><p>or drag and drop</p>
                              {/if}
                            </button>
                        </div>
                        {:else}
                        <div class="w-full h-full py-6 flex flex-col items-center gap-4">
                          {#each filepath as file, index }
                          <Fileitem title={file.split(/[/\\]/).pop()} focus={focusgrab(index)} index={index}/>
                          {/each}
                        </div>
                        {/if}
                    </div>
                </Pane>
                <Pane size={25}>
                    <Detailpanel file=true/>
                </Pane>
            </Splitpanes>
        </Pane>
        <Pane size={25} minSize={20}>
            <Propertiespanel/>
        </Pane>
      </Splitpanes>
</div>

<style global>
    :global(.splitpanes.my-theme) :global(.splitpanes__pane) {
    background-color: #ffffff;
  }
  :global(.splitpanes.my-theme) :global(.splitpanes__splitter) {
    background-color: #ffffff;
    box-sizing: border-box;
    position: relative;
    flex-shrink: 0;
  }
  :global(.splitpanes.my-theme) :global(.splitpanes__splitter:before), :global(.splitpanes.my-theme) :global(.splitpanes__splitter:after) {
    content: "";
    position: absolute;
    top: 50%;
    left: 50%;
    background-color: rgba(255, 255, 255, 0.15);
    transition: background-color 0.3s;
  }
  :global(.splitpanes.my-theme) :global(.splitpanes__splitter:hover:before), :global(.splitpanes.my-theme) :global(.splitpanes__splitter:hover:after) {
    background-color: rgba(0, 0, 0, 0.25);
  }
  :global(.splitpanes.my-theme) :global(.splitpanes__splitter:first-child) {
    cursor: auto;
  }
  
  :global(.my-theme.splitpanes) :global(.splitpanes) :global(.splitpanes__splitter) {
    z-index: 1;
  }
  :global(.my-theme.splitpanes--vertical) > :global(.splitpanes__splitter),
  :global(.my-theme) :global(.splitpanes--vertical) > :global(.splitpanes__splitter) {
    width: 7px;
    border-left: 1px solid #eee;
    cursor: col-resize;
  }
  :global(.my-theme.splitpanes--vertical) > :global(.splitpanes__splitter:before), :global(.my-theme.splitpanes--vertical) > :global(.splitpanes__splitter:after), :global(.my-theme) :global(.splitpanes--vertical) > :global(.splitpanes__splitter:before), :global(.my-theme) :global(.splitpanes--vertical) > :global(.splitpanes__splitter:after) {
    transform: translateY(-50%);
    width: 1px;
    height: 30px;
  }
  :global(.my-theme.splitpanes--vertical) > :global(.splitpanes__splitter:before),
  :global(.my-theme) :global(.splitpanes--vertical) > :global(.splitpanes__splitter:before) {
    margin-left: -2px;
  }
  :global(.my-theme.splitpanes--vertical) > :global(.splitpanes__splitter:after),
  :global(.my-theme) :global(.splitpanes--vertical) > :global(.splitpanes__splitter:after) {
    margin-left: 1px;
  }
  :global(.my-theme.splitpanes--horizontal) > :global(.splitpanes__splitter),
  :global(.my-theme) :global(.splitpanes--horizontal) > :global(.splitpanes__splitter) {
    height: 7px;
    border-top: 1px solid #eee;
    cursor: row-resize;
  }
  :global(.my-theme.splitpanes--horizontal) > :global(.splitpanes__splitter:before), :global(.my-theme.splitpanes--horizontal) > :global(.splitpanes__splitter:after), :global(.my-theme) :global(.splitpanes--horizontal) > :global(.splitpanes__splitter:before), :global(.my-theme) :global(.splitpanes--horizontal) > :global(.splitpanes__splitter:after) {
    transform: translateX(-50%);
    width: 30px;
    height: 1px;
  }
  :global(.my-theme.splitpanes--horizontal) > :global(.splitpanes__splitter:before),
  :global(.my-theme) :global(.splitpanes--horizontal) > :global(.splitpanes__splitter:before) {
    margin-top: -2px;
  }
  :global(.my-theme.splitpanes--horizontal) > :global(.splitpanes__splitter:after),
  :global(.my-theme) :global(.splitpanes--horizontal) > :global(.splitpanes__splitter:after) {
    margin-top: 1px;
  }
</style>
  
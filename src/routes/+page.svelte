<script lang="ts">
  import { Pane, Splitpanes } from 'svelte-splitpanes';
  import { Titlebar, Toolbar, Menubar, Detailpanel, Propertiespanel, Fileitem } from '../components/index'
  import { Tabs } from "melt/builders";
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';


  const tabIds = ["Output Setting", "Metadata"];
  type TabId = (typeof tabIds)[number];
  const tabs = new Tabs<TabId>({
		value: "Tab 1",
		orientation: 'vertical',
	});
  const Outputpath = 'C:/Users/DFM-RENDERING/Desktop';

  let inner = $state<HTMLElement>();
  let isDragging = $state(false);
  let focusedfile: number = $state(-1);
  let isfilefocused: boolean = $state(false);
  let videoInfo: VideoInfo[] = $state([]);


  interface VideoInfo {
    duration: string;
    format: string;
    size_bytes: number;
    file_path: string;
    file_name: string;
    width: number;
    height: number;
    bitrate_formated: string;
    aspect_ratio: String;
    frame_rate: number;
    thumbnail: string;
  }

  async function addfile(){
    const file = await open({multiple: true, directory: false,
      filters: [{ 
        name: 'Video', 
        extensions: ["mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "mpg", "mpeg", "3gp", "ogv"]
    }]
  });
   if (Array.isArray(file)) {
    for (const filepath of file) {
      try {
        const info: VideoInfo = await invoke("probe_video_detail", { path: filepath });
        info.thumbnail = await loadThumbnail(info.thumbnail);
        videoInfo.push(info);
      } catch (e) {
        console.error(`Failed to get info for ${filepath}`, e);
      }
    }
  } else if (file) {
    try {
      const info: VideoInfo = await invoke("get_video_info", { path: file });
      info.thumbnail = await loadThumbnail(info.thumbnail);
      videoInfo = [info];
    } catch (e) {
      console.error(`Failed to get info for ${file}`, e);
    }
  }
}

async function loadThumbnail(path: string) {
  const imageBytes: number[] = await invoke('get_thumbnail_image', { path });
  const blob = new Blob([new Uint8Array(imageBytes)], { type: 'image/jpeg' });
  return URL.createObjectURL(blob);
}

async function metadata(path: string) {
  await invoke('get_metadata', {path})
}

async function remux() {
  for (const element of videoInfo) {
    await invoke('remux', { filePath: element.file_path, outputPath: Outputpath });
  }
}
  

function draghandle(){ isDragging = !isDragging;}

function removefile() {
  if(focusedfile !== -1){
    if(videoInfo.length > 0)
      videoInfo.splice(focusedfile, 1);
  }
}

function focusgrab(index: number) {
  isfilefocused = true;
  focusedfile = index;
  console.log(focusedfile);
  metadata(videoInfo[focusedfile].file_path);
}

</script>

<div class="flex flex-col">
    <div>
        <Titlebar/>
        <Menubar addfile={addfile}/>
        <Toolbar addfile={addfile} removefile={removefile} convertfile={remux}/>
    </div>
</div>
<div class="h-full box-border overflow-hidden">
    <Splitpanes theme="my-theme">
        <Pane  size={65} minSize={20}>
          <div class="dot w-full h-full flex justify-center items-center">
            {#if videoInfo.length === 0}
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
              <div class="w-full h-full py-6 flex flex-col items-center gap-4 overflow-y-auto">
                {#each videoInfo as info, index}
                <Fileitem 
                  title={info.file_name} 
                  focus={focusgrab(index)} 
                  thumbnail={info.thumbnail} 
                  index={index}
                  />
                {/each}
              </div>
              {/if}
          </div>   
        </Pane>
        <Pane size={35} minSize={20}>
          <div class="h-full flex flex-row ">
            <div class="flex flex-wrap flex-col overflow-y-clip py-4 "
              {...tabs.triggerList}>
            {#each tabIds as id}
              <button class="cursor-pointer text-ellipsis whitespace-nowrap text-sm outline-none py-0.5"
                {...tabs.getTrigger(id)}>
                <div style="writing-mode: vertical-rl;"
                  class="overflow-clip rounded-sm px-2 py-1 ">
						      {id}
					      </div>
              </button>
            {/each}
            </div>
            {#each tabIds as id}
              <div class="h-full w-full" {...tabs.getContent(id)}>
                {#if id === "Output Setting"}
                <Propertiespanel/>
                {:else if id === "Metadata"}
                <Detailpanel
                  file={isfilefocused && videoInfo.length > 0}
                  filepath = {videoInfo[focusedfile].file_path}
                  duration={videoInfo[focusedfile].duration}
                  format={videoInfo[focusedfile].format}
                  size={((videoInfo[focusedfile].size_bytes ?? 0) / (1024*1024)).toFixed(2)}
                  width={videoInfo[focusedfile].width}
                  height={videoInfo[focusedfile].height}
                  framerate={videoInfo[focusedfile].frame_rate}
                  bitrate={videoInfo[focusedfile].bitrate_formated}
                  aspectratio={videoInfo[focusedfile].aspect_ratio}
                  />
                {/if}
              </div>
            {/each}
          </div>
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
  
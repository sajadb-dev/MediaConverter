<script lang="ts">
  import { Pane, Splitpanes } from 'svelte-splitpanes';
  import Detailpanel from '../components/detailpanel.svelte';
  import Propertiespanel from '../components/propertiespanel.svelte';
  import Fileitem from '../components/fileitem.svelte';
  import Toolbar from '../components/toolbar.svelte';
  import Menubar from '../components/menubar.svelte';
  import Titlebar from '../components/titlebar.svelte';
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';

  let isDragging = $state(false);
  let focusedfile: number = $state(0);
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
  console.log(URL.createObjectURL(blob))
  return URL.createObjectURL(blob);
}

function draghandle(){ isDragging = !isDragging;}

function removefile() {
  if(videoInfo.length > 0)
  videoInfo.splice(focusedfile, 1);
}

function focusgrab(index: number) {
  isfilefocused = true;
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
                <Pane size={70} minSize={60}>
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
                <Pane size={30}>
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
                      thumbnail={videoInfo[focusedfile].thumbnail}
                      />
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
  
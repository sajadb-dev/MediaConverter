<script lang="ts">
  import { Pane, Splitpanes } from 'svelte-splitpanes';
  import { Titlebar, Toolbar, Menubar, Detailpanel, Propertiespanel, Fileitem } from '../components/index'
  import { Tabs } from "melt/builders";
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import type { SvelteComponent } from 'svelte';
  import { listen } from '@tauri-apps/api/event';


  const tabIds = ["Output Setting", "Metadata"];
  type TabId = (typeof tabIds)[number];
  const tabs = new Tabs<TabId>({
		value: "Output Setting",
		orientation: 'vertical',
	});

  let Outputpath = 'C:/Users/DFM-RENDERING/Desktop/';

  let isDragging = $state(false);
  let focusedfile: number = $state(-1);
  let convertingfile: number = $state(0);
  let isfilefocused: boolean = $state(false);
  let videoInfo: VideoInfo[] = $state([]);
  let videoMetadata: any = $state([]);
  let settingpanel: SvelteComponent | undefined = $state();


  $inspect(videoInfo);
  $inspect(convertingfile);


  const codecList = [
    { value: "libx264", label: "H.264" },
    { value: "libx265", label: "H.265/HEVC" },
    { value: "mpeg1video", label: "MPEG-1" },
    { value: "mpeg2video", label: "MPEG-2" },
    { value: "mpeg4", label: "MPEG-4" },
    { value: "libaom-av1", label: "AV1" },
    { value: "libvpx-vp9", label: "WebM/VP9" },
    { value: "libtheora", label: "Theora" },
  ];

  const containerList = [
    { value: "mkv", label: "Matroska (.mkv)" },
    { value: "webm", label: "WebM (.webm)" },
    { value: "mpeg", label: "MPEG-1 (.mpg)" },
    { value: "mpegts", label: "MPEG-2 TS (.ts)" },
    { value: "mp4", label: "MPEG-4 (.mp4)" },
    { value: "ogg", label: "OGG (.ogg)" },
    { value: "avi", label: "AVI (.avi)" },
    { value: "mov", label: "QuickTime/MOV (.mov)" },
    { value: "flv", label: "Flash Video (.flv)" },
    { value: "asf", label: "ASF/WMV (.asf/.wmv)" },
    { value: "3gp", label: "3GP (.3gp)" },
  ];

  const encodingSpeed = [
    { value: "preset=slow", label: "Slowest" },
    { value: "preset=medium", label: "Good" },
    { value: "preset=ultrafast", label: "Realtime" },
  ];

  interface VideoInfo {
    input_path: string;
    output_path: string | undefined;
    duration: number,
    codec: string;
    container: string;
    encodingspeed: string;
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
        const meta: any = await invoke('get_metadata', {path: filepath});
        let tmp: VideoInfo = {
          input_path: meta.file_path,
          output_path: undefined,
          duration: meta.duration,
          codec: 'same',
          container: 'same',
          encodingspeed: 'Good'
        };
        videoMetadata.push(meta);
        videoInfo.push(tmp);
      } catch (e) {
        console.error(`Failed to get info for ${filepath}`, e);
      }
    }
  } else if (file) {
    try {
      const meta: any = await invoke('get_metadata', {path: file});
      let tmp: VideoInfo = {
          input_path: meta.file_path,
          output_path: undefined,
          duration: meta.duration,
          codec: 'same',
          container: 'same',
          encodingspeed: 'Good'
        };
      videoMetadata = [meta];
      videoInfo = [tmp]
    } catch (e) {
      console.error(`Failed to get info for ${file}`, e);
    }
  }
}

async function remux(InputPath: string , Outputpath: string, duration: number) {
  await invoke('remux', { inputPath: InputPath, outputPath: Outputpath, duration: duration });  
}

async function transcode(inputPath: string, outputPath: string, codecName: string, codecOpts: string) {
  await invoke("transcode_video", {
    inputPath,
    outputPath,
    codecName,
    codecOpts
  });
}

function valuechangeHandle(i: string, e: any) {
  if(i === "container")
    videoInfo[focusedfile].container = e;
  else if (i === "codec")
    videoInfo[focusedfile].codec = e;
  else if (i === "encodingspeed")
    videoInfo[focusedfile].encodingspeed = e;
}

listen('remux-start', () => {
  console.log('Remuxing started!');
});

listen<number>('ffmpeg-progress', (event) => {
  videoMetadata[convertingfile].progress = event.payload.toFixed(2);
});

listen('remux-complete', () => {
  videoMetadata[convertingfile].progress = 100;
  convertingfile = convertingfile + 1;
});

function outputpathChangehandler(output_path: any, output_title: any) {
  videoInfo[focusedfile].output_path = `${output_path}\\${output_title}.`;;
}

function removeFileExtension(file_name: string) {
  const lastDotIndex = file_name.lastIndexOf('.');
  if (lastDotIndex !== -1) {
    return file_name.slice(0, lastDotIndex);
  }
  return file_name; // No file extension found
}

function makeoutputpath(inputpath: string,container: string) {
  let filename = removeFileExtension(inputpath.split('\\').pop()?.split('/').pop() || '');
  console.log(filename);
  let format = containerList.find((entry) => entry.label === container)?.value;
  console.log(format);
  let temp = `${Outputpath}${filename}.${format}`;
  return temp;
}

function convert() {
  videoInfo.forEach(elm => {
    let temp_path
      if(elm.output_path === undefined)
        temp_path = makeoutputpath(elm.input_path,elm.container);
      else
        temp_path = makeoutputpath(elm.output_path,elm.container);
    if(elm.container !== "same" && elm.codec === "same") {
      remux(elm.input_path, temp_path, elm.duration);
    }
    else if (elm.codec !== "same") {
      let codecEntry = codecList.find((entry) => entry.label === elm.codec);
      let codec = codecEntry ? codecEntry.value : '';
      let encodespeed = encodingSpeed.find((entry) => entry.label === elm.encodingspeed)?.value;
      console.log(encodespeed);
      transcode(elm.input_path,temp_path,codec,`${encodespeed}, crf=18`);
    }
  });
}

function draghandle(){ isDragging = !isDragging;}

function removefile() {
  if(focusedfile !== -1){
    if(videoMetadata.length > 0)
      videoMetadata.splice(focusedfile, 1);
      videoInfo.splice(focusedfile, 1);
      isfilefocused = false;
  }
}

function activefileitem(i: number) {
  const tmp = document.querySelector(`.file-item-${i}`);
  tmp?.classList.add('ring-3');
}
function deactivefileitem(i: number) {
  const tmp = document.querySelector(`.file-item-${i}`);
  tmp?.classList.remove('ring-3');
}

function focusgrab(index: number) {
  isfilefocused = true;
  deactivefileitem(focusedfile);
  focusedfile = index;
  console.log(focusedfile);
  activefileitem(focusedfile);
  settingpanel?.setselectValues();
}

</script>

<div class="flex flex-col">
    <div>
        <Titlebar/>
        <Menubar addfile={addfile}/>
        <Toolbar addfile={addfile} removefile={removefile} convertfile={convert}/>
    </div>
</div>
<div class="h-full box-border overflow-hidden">
    <Splitpanes theme="my-theme">
        <Pane  size={65} minSize={20}>
          <div class="dot w-full h-full flex justify-center items-center">
            {#if videoMetadata.length === 0}
            <div class="w-full h-full flex justify-center items-center">
                  <button 
                      class="relative w-9/12 min-h-9/12 flex justify-center items-center rounded-2xl border-2 border-dashed bg-[var(--filedrop-color)] border-[var(--outline)] cursor-pointer"
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
                {#each videoMetadata as info, index}
                <Fileitem
                  classnames={`file-item-${index}`}
                  title={info.file_name} 
                  focus={focusgrab(index)} 
                  filepath={info.file_path}
                  progressvalue={info.progress}
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
              <button class="group cursor-pointer text-ellipsis whitespace-nowrap text-sm outline-none py-0.5"
                {...tabs.getTrigger(id)}>
                <div style="writing-mode: vertical-rl;"
                  class="border border-[var(--tab-border)] group-data-[active]:bg-[var(--tab-active)] group-data-[active]:border-[var(--tab-active-border)] overflow-clip rounded-l-sm px-2 py-1 ">
						      {id}
					      </div>
              </button>
            {/each}
            </div>
            {#each tabIds as id}
              <div class="h-full w-full border-l border-[var(--outline)]" {...tabs.getContent(id)}>
                {#if id === "Output Setting"}
                <Propertiespanel
                  file={isfilefocused && videoMetadata.length > 0}
                  valuechange={valuechangeHandle}
                  outputpathChange={outputpathChangehandler}
                  videoinfo={videoInfo[focusedfile]}
                  bind:this={settingpanel}/>
                {:else if id === "Metadata"}
                <Detailpanel
                  file={isfilefocused && videoMetadata.length > 0}
                  metadata={videoMetadata[focusedfile]}
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
    background-color: var(--background);
  }
  :global(.splitpanes.my-theme) :global(.splitpanes__splitter) {
    background-color: var(--background);
    box-sizing: border-box;
    position: relative;
    flex-shrink: 0;
  }
  :global(.splitpanes.my-theme) :global(.splitpanes__splitter:before), :global(.splitpanes.my-theme) :global(.splitpanes__splitter:after) {
    content: "";
    position: absolute;
    top: 50%;
    left: 50%;
    background-color: var(--outline);
    transition: background-color 0.3s;
  }
  :global(.splitpanes.my-theme) :global(.splitpanes__splitter:hover:before), :global(.splitpanes.my-theme) :global(.splitpanes__splitter:hover:after) {
    background-color: rgba(255, 255, 255, 0.25);
  }
  :global(.splitpanes.my-theme) :global(.splitpanes__splitter:first-child) {
    cursor: auto;
  }
  
  :global(.my-theme.splitpanes) :global(.splitpanes) :global(.splitpanes__splitter) {
    z-index: 1;
  }
  :global(.my-theme.splitpanes--vertical) > :global(.splitpanes__splitter),
  :global(.my-theme) :global(.splitpanes--vertical) > :global(.splitpanes__splitter) {
    width: 2px;
    border-left: 1px solid var(--outline);
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
    border-top: 1px solid var(--outline);
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
  
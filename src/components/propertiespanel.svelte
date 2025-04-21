<script lang="ts">
  import { Select } from "melt/builders";
  import { open } from "@tauri-apps/plugin-dialog";

  let { file, videoinfo, valuechange, outputpathChange} = $props();
  let outputpath: HTMLInputElement | undefined = $state();
  let outputtitle: HTMLInputElement | undefined = $state();

  
  async function selectDirectory() {
    const selected = await open({
      directory: true,
      multiple: false, // set to true if you want multiple folders
      title: 'Select a directory'
    });
    outputpath.value = selected;
    outputpathChange(outputpath?.value, outputtitle?.value );
  }


  export const setselectValues = () => {
    if(videoinfo) {
      selectcontainer.select(videoinfo.container);
      selectcodec.select(videoinfo.codec);
      selectencodingspeed.select(videoinfo.encodingspeed);
      if(outputpath) {
        if(videoinfo.output_path === undefined)
          outputpath.value = "";
        else
          outputpath.value = videoinfo.output_path;
      }
      if(outputtitle) {
        if(videoinfo.output_path === undefined)
          outputtitle.value = "";
        else {
          const filename = videoinfo.output_path.split(/[\\/]/).pop();
          const dot = filename.lastIndexOf(".");
          outputtitle.value = dot > 0 ? filename.slice(0, dot) : filename;
        }
      }
    }
  }

    function Containervaluechange() {
      valuechange("container",selectcontainer.value);
    }
    function CodecValueChange() {
      valuechange("codec",selectcodec.value);
    }
    function encodingSpeedValueChange() {
      valuechange("encodingspeed",selectencodingspeed.value);
    }


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

  const encodingSpeedList = [
    { value: "1", label: "Slowest" },
    { value: "2", label: "Good" },
    { value: "3", label: "Realtime" },
  ];

const codecoptions = [
    "same",
    "H.264",
    "H.265/HEVC",
    "MPEG-1",
    "MPEG-2",
    "MPEG-4",
    "AV1",
    "WebM/VP9",
    "Theora"
] as const;
type CodecOption = (typeof codecoptions)[number];

const containeroptions = [
  "same",
  "Matroska (.mkv)",
  "WebM (.webm)",
  "MPEG-1 (.mpg)",
  "MPEG-2 TS (.ts)",
  "MPEG-4 (.mp4)",
  "OGG (.ogg)",
  "AVI (.avi)",
  "QuickTime/MOV (.mov)",
  "Flash Video (.flv)",
  "ASF/WMV (.asf/.wmv)",
  "3GP (.3gp)"
] as const;
  type ContainerOption = (typeof containeroptions)[number];

  const encodingspeedoption = [
    "Slowest",
    "Good",
    "Real Time"
] as const;
type EncodingSpeecOption = (typeof encodingspeedoption)[number];

    const selectcontainer = new Select<ContainerOption>({
      onValueChange: Containervaluechange,
    });
    const selectcodec = new Select<CodecOption>({
      onValueChange: CodecValueChange,
    });
    const selectencodingspeed = new Select<EncodingSpeecOption>({
      onValueChange: encodingSpeedValueChange,
    });

</script>

<div class="w-full h-full bg-[var(--button-secondary)]/60">
  <div class="w-fit p-2 text-sm">Output Setting:</div>
  {#if file}
  <div class="w-full h-full mt-2 px-2">

    <label for="outputpath" class="text-xs font-bold">Output Path:</label>
    <div class="w-full h-6 flex justify-between px-2 rounded bg-[var(--input-background)]">
      <input bind:this={outputpath} class="w-full truncate" id="outputpath" type="text" oninput={() => outputpathChange(outputpath?.value,outputtitle?.value)}/>
      <button class=" min-w-28 bg-[var(--select-color)] hover:bg-[var(--select-color)]/60 hover:cursor-pointer px-2 rounded-md text-sm" onclick={selectDirectory}>Select Folder</button>
    </div>
    

    <label for="videotitle" class="text-xs font-bold">Output Title:</label>
    <input bind:this={outputtitle} id="videotitle" class="w-full h-6 px-2 rounded bg-[var(--input-background)] truncate" type="text" oninput={() => outputpathChange(outputpath?.value,outputtitle?.value)}/>

    <!-- Select for Container -->
    <label for={selectcontainer.ids.trigger} class="text-xs font-bold">Containers</label>
    <button {...selectcontainer.trigger}
        class="w-full flex items-center justify-between overflow-hidden rounded border border-[var(--outline)] bg-[var(--select-color)] pl-3 pr-4 py-1 text-left
				hover:cursor-pointer hover:bg-[var(--select-color)]/40 active:bg-[var(--select-color)]/10">
      {selectcontainer.value ?? "Select a container"}
    </button>
    <div {...selectcontainer.content} class="p-2 border-[var(--outline)] border rounded bg-[var(--select-color)] text-sm">
      {#each containeroptions as option}
        <div {...selectcontainer.getOption(option)} class="px-2 py-1 rounded hover:bg-[var(--outline)] text-white cursor-pointer">
          {option}
        </div>
      {/each}
    </div>

    <!-- Select for Codecs -->
    <label for={selectcodec.ids.trigger} class="text-xs font-bold">Codecs</label>
    <button {...selectcodec.trigger}
        class="w-full flex items-center justify-between overflow-hidden rounded border border-[var(--outline)] bg-[var(--select-color)] pl-3 pr-4 py-1 text-left
				hover:cursor-pointer hover:bg-[var(--select-color)]/40 active:bg-[var(--select-color)]/10">
      {selectcodec.value ?? "Select a Codec"}
    </button>

    <div {...selectcodec.content} class="p-2 border-[var(--outline)] border rounded bg-[var(--select-color)] text-sm">
      {#each codecoptions as option}
        <div {...selectcodec.getOption(option)} class="px-2 py-1 rounded hover:bg-[var(--outline)] text-white cursor-pointer">
          {option}
        </div>
      {/each}
    </div>

    <!-- Select for Encoding Speed -->
    <label for={selectencodingspeed.ids.trigger} class="text-xs font-bold">Encoding Speed</label>
    <button {...selectencodingspeed.trigger}
        class="w-full flex items-center justify-between overflow-hidden rounded border border-[var(--outline)] bg-[var(--select-color)] pl-3 pr-4 py-1 text-left
				hover:cursor-pointer hover:bg-[var(--select-color)]/40 active:bg-[var(--select-color)]/10">
      {selectencodingspeed.value ?? "Select Encoding Speed"}
    </button>

    <div {...selectencodingspeed.content} class="p-2 border-[var(--outline)] border rounded bg-[var(--select-color)] text-sm">
      {#each encodingspeedoption as option}
        <div {...selectencodingspeed.getOption(option)} class="px-2 py-1 rounded hover:bg-[var(--outline)] text-white cursor-pointer">
          {option}
        </div>
      {/each}
    </div>
  </div>
  {/if}
</div>

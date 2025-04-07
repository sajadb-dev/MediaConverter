<script lang="ts">
    import { Tooltip } from "melt/builders";
    
    let { addfile , removefile } = $props();
    
    const addtooltip = new Tooltip({closeOnPointerDown : true, disableHoverableContent : true});
    const removetooltip = new Tooltip({closeOnPointerDown : true, disableHoverableContent : true});
    const converttooltip = new Tooltip({closeOnPointerDown : true, disableHoverableContent : true});
    const pausetooltip = new Tooltip({closeOnPointerDown : true, disableHoverableContent : true});
    const canceltooltip = new Tooltip({closeOnPointerDown : true, disableHoverableContent : true});
</script>

<div class="h-14 px-4 flex items-center border-b border-slate-200 gap-6">
    <div class="flex gap-4">
        <button {...addtooltip.trigger} type="button" class="w-24 h-10 font-bold rounded bg-blue-300 hover:cursor-pointer hover:bg-blue-500 active:bg-blue-600" onclick={addfile}>ADD</button>
        <div {...addtooltip.content} class="bg-slate-100 rounded-sm p-2">
            <div {...addtooltip.arrow}></div>
            <p class="text-sm">add media file</p>
          </div>
        <button {...removetooltip.trigger} type="button" class="w-10 h-10 rounded flex items-center justify-center bg-slate-200 hover:cursor-pointer hover:bg-red-600 active:bg-red-700 hover:text-white active:text-white" aria-label="remove" onclick={removefile}>
            <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" d="M6.5 7a.5.5 0 0 1 .5.5v4a.5.5 0 0 1-1 0v-4a.5.5 0 0 1 .5-.5m3.5.5a.5.5 0 0 0-1 0v4a.5.5 0 0 0 1 0z"/><path fill="currentColor" fill-rule="evenodd" d="M8.12 0h-.241c-.402 0-.735 0-1.01.022c-.281.023-.539.072-.782.196a2 2 0 0 0-.874.874c-.124.243-.173.501-.196.782c-.022.271-.022.603-.022 1.01v.121h-3.5a.5.5 0 0 0 0 1h.54l.597 7.16c.063.758.113 1.36.192 1.84c.08.492.196.9.41 1.27c.354.604.88 1.09 1.51 1.39c.386.184.803.265 1.3.304c.487.038 1.09.038 1.85.038h.21c.76 0 1.36 0 1.85-.038c.497-.039.913-.12 1.3-.304a3.5 3.5 0 0 0 1.51-1.39c.216-.368.33-.777.41-1.27c.08-.48.129-1.08.193-1.84l.597-7.16h.54a.5.5 0 0 0 0-1h-3.5v-.12c0-.403 0-.736-.022-1.01c-.023-.282-.072-.54-.196-.783a2 2 0 0 0-.874-.874C9.669.094 9.41.045 9.13.022C8.859 0 8.527 0 8.12 0M3.04 4l.588 7.06c.065.784.112 1.34.184 1.78c.07.432.159.705.287.924a2.5 2.5 0 0 0 1.08.992c.229.11.509.175.945.21c.442.035 1 .035 1.79.035h.167c.787 0 1.35 0 1.79-.035c.436-.034.716-.1.945-.21a2.5 2.5 0 0 0 1.08-.992c.128-.219.217-.493.288-.924c.071-.438.118-.997.184-1.78L12.956 4h-9.91zM6 3h4v-.1c0-.428 0-.719-.019-.944c-.018-.219-.05-.331-.09-.41a1 1 0 0 0-.437-.437c-.078-.04-.19-.072-.41-.09A13 13 0 0 0 8.1 1h-.2c-.428 0-.719 0-.944.019c-.219.018-.331.05-.41.09a1 1 0 0 0-.437.437c-.04.078-.072.19-.09.41C6 2.181 6 2.472 6 2.9z" clip-rule="evenodd"/></svg>
        </button>
        <div {...removetooltip.content} class="bg-slate-100 rounded-sm p-2">
            <div {...removetooltip.arrow}></div>
            <p class="text-sm">remove media from list</p>
          </div>
    </div>
    <div class="h-8/12 border-r border-slate-300"></div>
    <div class="flex gap-4">
        <button {...converttooltip.trigger} type="button" class="w-10 h-10 rounded flex items-center justify-center bg-slate-200 hover:cursor-pointer hover:bg-lime-400 active:bg-lime-500 hover:text-white active:text-white" aria-label="pause">
            <div {...converttooltip.content} class="bg-slate-100 rounded-sm p-2">
                <div {...converttooltip.arrow}></div>
                <p class="text-sm">convert selected media files</p>
              </div>
            <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" d="M5.85.854a.5.5 0 0 0-.707-.707l-3 3a.5.5 0 0 0 0 .707l3 3a.5.5 0 0 0 .707-.707L3.7 3.997h7.79a2.5 2.5 0 0 1 2.5 2.5v2a.5.5 0 0 0 1 0v-2c0-1.93-1.57-3.5-3.5-3.5H3.7L5.85.847zM2 7.5a.5.5 0 0 0-1 0v2C1 11.43 2.57 13 4.5 13h7.79l-2.15 2.15a.5.5 0 0 0 .707.707l3-3a.5.5 0 0 0 0-.707l-3-3a.5.5 0 0 0-.707.707l2.15 2.15H4.5a2.5 2.5 0 0 1-2.5-2.5v-2z"/></svg>
        </button>
        <button {...pausetooltip.trigger} type="button" class="w-10 h-10 rounded flex items-center justify-center bg-slate-200 hover:cursor-pointer hover:bg-yellow-500 active:bg-orange-400 hover:text-white active:text-white" aria-label="remove">
            <div {...pausetooltip.content} class="bg-slate-100 rounded-sm p-2">
                <div {...pausetooltip.arrow}></div>
                <p class="text-sm">pause convertion</p>
              </div>
            <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" fill-rule="evenodd" d="M5 2a1 1 0 0 1 1 1v10a1 1 0 0 1-2 0V3a1 1 0 0 1 1-1m6 0a1 1 0 0 1 1 1v10a1 1 0 0 1-2 0V3a1 1 0 0 1 1-1" clip-rule="evenodd"/></svg>
        </button>
        <button {...canceltooltip.trigger} type="button" class="w-10 h-10 rounded flex items-center justify-center bg-slate-200 hover:cursor-pointer hover:bg-rose-500 active:bg-rose-700 hover:text-white active:text-white" aria-label="cancel">
            <div {...canceltooltip.content} class="bg-slate-100 rounded-sm p-2">
                <div {...canceltooltip.arrow}></div>
                <p class="text-sm">cancel convertion</p>
              </div>
            <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" d="m8 8.71l-4.15 4.15a.48.48 0 0 1-.352.146a.5.5 0 0 1-.361-.142a.5.5 0 0 1-.142-.361q0-.205.146-.352l4.15-4.15l-4.15-4.15a.48.48 0 0 1-.146-.356a.496.496 0 0 1 .502-.497q.205 0 .352.146L8 7.294l4.15-4.15a.48.48 0 0 1 .356-.146q.103 0 .19.039a.497.497 0 0 1 .161.814l-4.15 4.15l4.15 4.15a.496.496 0 0 1 0 .708a.5.5 0 0 1-.351.146a.48.48 0 0 1-.356-.146z"/></svg>
        </button>
    </div>
</div>
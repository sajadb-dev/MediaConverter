<script lang="ts">
import {getCurrentWindow} from '@tauri-apps/api/window'

const appWindow = getCurrentWindow();

function closewindow(){
    appWindow.close();
}
function maximizewindow(){
    appWindow.toggleMaximize();
}
function minimizewindow(){
    appWindow.minimize();
}

</script>

<div data-tauri-drag-region class="h-6 flex justify-between border-b border-slate-400">
  <div class="flex h-full mx-2 items-center gap-2">
    <div class="h-full flex items-center justify-center">
        <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16" {...$$props}><!-- Icon from Qlementine Icons by Olivier Cléro - https://github.com/oclero/qlementine-icons/blob/master/LICENSE --><path fill="currentColor" d="M12.5 4a.5.5 0 0 0 0-1h-9a.5.5 0 0 0 0 1z"/><path fill="currentColor" fill-rule="evenodd" d="M13 15c1.66 0 3-1.34 3-3V4c0-1.66-1.34-3-3-3H3C1.34 1 0 2.34 0 4v8c0 1.66 1.34 3 3 3zM3 14h10c1.1 0 2-.895 2-2V6H1v6c0 1.1.895 2 2 2M15 4v1H1V4c0-1.1.895-2 2-2h10c1.1 0 2 .895 2 2" clip-rule="evenodd"/></svg>
    </div>
    <span class="text-sm font-lobster">Media Converter</span>
  </div>
  <div class="flex">
    <button 
      id= "minimizeBtn" 
      class= "h-full aspect-square flex justify-center items-center bg-transparent hover:bg-gray-400 active:bg-gray-600" 
      title= "Minimize" 
      type= "button"
      aria-label="minimize"
      onclick={minimizewindow}>
      <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" d="M3.5 8a.496.496 0 0 1-.351-.848A.48.48 0 0 1 3.5 7h9q.103 0 .19.039a.5.5 0 0 1 .161.112a.5.5 0 0 1 .146.35a.5.5 0 0 1-.146.352A.5.5 0 0 1 12.5 8z"/></svg>
    </button>
    <button 
      id= "maximizeBtn" 
      class="h-full aspect-square flex justify-center items-center bg-transparent hover:bg-gray-400 active:bg-gray-600" 
      title=''
      type="button"
      aria-label="maximize"
      onclick={maximizewindow}>
      <label class="swap">
        <input type="checkbox" />
            <svg class="swap-on" xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" fill-rule="evenodd" d="M9.8 4H6.2c-.577 0-.949 0-1.23.024c-.272.022-.372.06-.422.085a1 1 0 0 0-.437.437c-.025.05-.063.15-.085.422c-.023.283-.024.656-.024 1.23v3.6c0 .577 0 .949.024 1.23c.022.272.06.372.085.422c.096.188.249.341.437.437c.05.025.15.063.422.085c.283.023.656.024 1.23.024h3.6c.577 0 .949 0 1.23-.024c.272-.022.372-.06.422-.085c.188-.096.341-.249.437-.437c.025-.05.063-.15.085-.422c.023-.283.024-.656.024-1.23v-3.6c0-.577 0-.949-.024-1.23c-.022-.272-.06-.372-.085-.422a1 1 0 0 0-.437-.437c-.05-.025-.15-.063-.422-.085C10.747 4 10.374 4 9.8 4m-6.58.092c-.218.428-.218.988-.218 2.11v3.6c0 1.12 0 1.68.218 2.11c.192.376.498.682.874.874c.428.218.988.218 2.11.218h3.6c1.12 0 1.68 0 2.11-.218c.376-.192.682-.498.874-.874c.218-.428.218-.988.218-2.11v-3.6c0-1.12 0-1.68-.218-2.11a2 2 0 0 0-.874-.874C11.486 3 10.926 3 9.804 3h-3.6c-1.12 0-1.68 0-2.11.218a2 2 0 0 0-.874.874" clip-rule="evenodd"/></svg>
            <svg class="swap-off" xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" d="M9.8 4H5.27c.193-.334.479-.606.824-.782C6.522 3 7.082 3 8.204 3h1.6c1.12 0 1.68 0 2.11.218c.376.192.682.498.874.874c.218.428.218.988.218 2.11v1.6c0 1.12 0 1.68-.218 2.11a2 2 0 0 1-.782.824v-4.53c0-.577 0-.949-.024-1.23c-.022-.272-.06-.372-.085-.422a1 1 0 0 0-.437-.437c-.05-.025-.15-.063-.422-.085a17 17 0 0 0-1.23-.024z"/><path fill="currentColor" fill-rule="evenodd" d="M3 8.2c0-1.12 0-1.68.218-2.11c.192-.376.498-.682.874-.874c.428-.218.988-.218 2.11-.218h1.6c1.12 0 1.68 0 2.11.218c.376.192.682.498.874.874c.218.428.218.988.218 2.11v1.6c0 1.12 0 1.68-.218 2.11a2 2 0 0 1-.874.874c-.428.218-.988.218-2.11.218h-1.6c-1.12 0-1.68 0-2.11-.218a2 2 0 0 1-.874-.874C3 11.482 3 10.922 3 9.8zM6.2 6h1.6c.577 0 .949 0 1.23.024c.272.022.372.06.422.085c.188.096.341.249.437.437c.025.05.063.15.085.422c.023.283.024.656.024 1.23v1.6c0 .577 0 .949-.024 1.23c-.022.272-.06.372-.085.422a1 1 0 0 1-.437.437c-.05.025-.15.063-.422.085c-.283.023-.656.024-1.23.024H6.2c-.577 0-.949 0-1.23-.024c-.272-.022-.372-.06-.422-.085a1 1 0 0 1-.437-.437c-.025-.05-.063-.15-.085-.422a17 17 0 0 1-.024-1.23v-1.6c0-.577 0-.949.024-1.23c.022-.272.06-.372.085-.422c.096-.188.249-.341.437-.437c.05-.025.15-.063.422-.085C5.253 6 5.626 6 6.2 6" clip-rule="evenodd"/></svg>
      </label>
    </button>
    <button 
      id= "closeBtn" 
      class= "h-full aspect-square flex justify-center items-center hover:bg-red-600 active:bg-red-800"
      title= "Close" 
      type= "button"
      aria-label="close"
      onclick={closewindow}>
      <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16" {...$$props}><!-- Icon from Qlementine Icons by Olivier Cléro - https://github.com/oclero/qlementine-icons/blob/master/LICENSE --><path fill="currentColor" d="m8 8.71l-4.15 4.15a.48.48 0 0 1-.352.146a.5.5 0 0 1-.361-.142a.5.5 0 0 1-.142-.361q0-.205.146-.352l4.15-4.15l-4.15-4.15a.48.48 0 0 1-.146-.356a.496.496 0 0 1 .502-.497q.205 0 .352.146L8 7.294l4.15-4.15a.48.48 0 0 1 .356-.146q.103 0 .19.039a.497.497 0 0 1 .161.814l-4.15 4.15l4.15 4.15a.496.496 0 0 1 0 .708a.5.5 0 0 1-.351.146a.48.48 0 0 1-.356-.146z"/></svg>
    </button>
  </div>
</div>

<script lang="ts">
  import {getCurrentWindow} from '@tauri-apps/api/window'
  import { createMenubar, melt } from '@melt-ui/svelte'
  
  const appWindow = getCurrentWindow();
  let { addfile } = $props();

  const {
    elements: { menubar },
    builders: { createMenu }
  } = createMenubar()

  const { elements : {menu, item, trigger} } = createMenu()
  const { elements : { menu: menuA, item: itemA, trigger: triggerA }} = createMenu()
  const { elements : { menu: menuB, item: itemB, trigger: triggerB }} = createMenu()

  function closewindow(){
    appWindow.close();
}
</script>

<div class="h-6 flex justify-between border-b bg-[var(--background)]/90 border-[var(--outline)]">
  <div {...$menubar} class="h-full flex gap-2 text-sm" use:menubar>
    <button {...$trigger} class="h-full w-16 flex items-center justify-center hover:bg-[var(--hover-color)] active:hover:bg-[var(--active-color)]" use:trigger>File</button>
    <div {...$menu} class="max-h-[300px] min-w-[220px] p-1 flex flex-col shadow-lg bg-[var(--background)] border border-[var(--outline)] text-sm rounded-sm ring-0" use:menu>
      <div {...$item} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-[var(--hover-color)] active:hover:bg-[var(--active-color)] select-none outline-none z-20" use:item on:m-click={addfile}>Add</div>
      <div {...$item} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-[var(--hover-color)] active:hover:bg-[var(--active-color)] select-none outline-none z-20" use:item>...</div>
      <div {...$item} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-[var(--hover-color)] active:hover:bg-[var(--active-color)] select-none outline-none z-20" use:item on:m-click={closewindow}>Exit</div>
    </div>
   
    <button {...$triggerA} class="h-full w-16 flex items-center justify-center hover:bg-[var(--hover-color)] active:hover:bg-[var(--active-color)]" use:triggerA>Edit</button>
    <div {...$menuA} class="max-h-[300px] min-w-[220px] p-1 flex flex-col shadow-lg bg-[var(--background)] border border-[var(--outline)] text-sm rounded-sm ring-0" use:menuA>
      <div {...$itemA} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-[var(--hover-color)] active:hover:bg-[var(--active-color)] select-none outline-none z-20" use:itemA>Convert</div>
      <div {...$itemA} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-[var(--hover-color)] active:hover:bg-[var(--active-color)] select-none outline-none z-20" use:itemA>...</div>
      <div {...$itemA} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-[var(--hover-color)] active:hover:bg-[var(--active-color)] select-none outline-none z-20" use:itemA>Properties</div>
    </div>
   
    <button {...$triggerB} class="h-full w-16 flex items-center justify-center hover:bg-[var(--hover-color)] active:hover:bg-[var(--active-color)]" use:triggerB>Help</button>
    <div {...$menuB} class="max-h-[300px] min-w-[220px] p-1 flex flex-col shadow-lg bg-[var(--background)] border border-[var(--outline)] text-sm rounded-sm ring-0" use:menuB>
      <div {...$itemB} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-[var(--hover-color)] active:hover:bg-[var(--active-color)] select-none outline-none z-20" use:itemB>...</div>
      <div {...$itemB} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-[var(--hover-color)] active:hover:bg-[var(--active-color)] select-none outline-none z-20" use:itemB>...</div>
      <div {...$itemB} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-[var(--hover-color)] active:hover:bg-[var(--active-color)] select-none outline-none z-20" use:itemB>...</div>
    </div>
  </div>
</div>
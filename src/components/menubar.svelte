<script lang="ts">
  import { createMenubar, melt } from '@melt-ui/svelte'

  const {
    elements: { menubar },
    builders: { createMenu }
  } = createMenubar()
  const { elements : {menu, item, trigger} } = createMenu()
  const { elements : { menu: menuA, item: itemA, trigger: triggerA }} = createMenu()
  const { elements : { menu: menuB, item: itemB, trigger: triggerB }} = createMenu()
</script>

<div class="h-6 flex justify-between border-b border-slate-200">
  <div {...$menubar} class="h-full flex gap-2 text-sm" use:menubar>
    <button {...$trigger} class="h-full w-16 flex items-center justify-center hover:bg-slate-100 active:hover:bg-slate-200" use:trigger>File</button>
    <div {...$menu} class="max-h-[300px] min-w-[220px] p-1 flex flex-col shadow-lg bg-white text-sm rounded-sm ring-0" use:menu>
      <div {...$item} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-slate-100 select-none outline-none z-20" use:item>Add</div>
      <div {...$item} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-slate-100 select-none outline-none z-20" use:item>...</div>
      <div {...$item} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-slate-100 select-none outline-none z-20" use:item>Exit</div>
    </div>
   
    <button {...$triggerA} class="h-full w-16 flex items-center justify-center hover:bg-slate-100 active:hover:bg-slate-200" use:triggerA>Edit</button>
    <div {...$menuA} class="max-h-[300px] min-w-[220px] p-1 flex flex-col shadow-lg bg-white text-sm rounded-sm ring-0" use:menuA>
      <div {...$itemA} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-slate-100 select-none outline-none z-20" use:itemA>Convert</div>
      <div {...$itemA} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-slate-100 select-none outline-none z-20" use:itemA>...</div>
      <div {...$itemA} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-slate-100 select-none outline-none z-20" use:itemA>Properties</div>
    </div>
   
    <button {...$triggerB} class="h-full w-16 flex items-center justify-center hover:bg-slate-100 active:hover:bg-slate-200" use:triggerB>Help</button>
    <div {...$menuB} class="max-h-[300px] min-w-[220px] p-1 flex flex-col shadow-lg bg-white text-sm rounded-sm ring-0" use:menuB>
      <div {...$itemB} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-slate-100 select-none outline-none z-20" use:itemB>...</div>
      <div {...$itemB} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-slate-100 select-none outline-none z-20" use:itemB>...</div>
      <div {...$itemB} class="h-6 min-h-[24px] pl-6 pr-1 rounded-sm hover:bg-slate-100 select-none outline-none z-20" use:itemB>...</div>
    </div>
  </div>
</div>
<script lang="ts">
  import "../app.css";
  import { convertFileSrc, invoke } from "@tauri-apps/api/core";
  import { fade } from "svelte/transition";
  import {
    createVirtualizer,
    createWindowVirtualizer,
  } from "@tanstack/svelte-virtual";
  import { onMount } from "svelte";
  import { type Event as TauriEvent, listen } from "@tauri-apps/api/event";

  const columns = 3;
  let timeout: number;

  type Image = { name: string; path: string };

  let wallpapers = $state<Image[]>([]);

  let virtualListEl = $state<HTMLDivElement>();
  let virtualItemEls = $state<HTMLDivElement[]>([]);
  let gridEl = $state<HTMLDivElement>();
  let selectedIndex = $state<number>(0);

  let prevRowCount = $state(0);
  let search = $state<string>("");
  let filterd = $derived(
    wallpapers.filter((v) => v.name.toLowerCase().includes(search)),
  );

  function onSearch(e: Event) {
    const target = e.target as HTMLInputElement;
    const v = target.value.toLowerCase();
    clearTimeout(timeout);
    timeout = setTimeout(() => (search = v), 500);
  }
  function handleKeyDown(e: KeyboardEvent) {
    const maxIndex = filterd.length - 1;

    switch (e.key) {
      case "l":
      case "ArrowRight":
        e.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, maxIndex);
        scrollToSelected();
        break;
      case "h":
      case "ArrowLeft":
        e.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        scrollToSelected();
        break;
      case "j":
      case "ArrowDown":
        e.preventDefault();
        selectedIndex = Math.min(selectedIndex + columns, maxIndex);
        scrollToSelected();
        break;
      case "k":
      case "ArrowUp":
        e.preventDefault();
        selectedIndex = Math.max(selectedIndex - columns, 0);
        scrollToSelected();
        break;
      case "Enter":
        e.preventDefault();
        if (selectedIndex >= 0) {
          // Handle selection - you can emit an event or call a function
          console.log("Selected:", filterd[selectedIndex]);
        }
        break;
      case "Escape":
        selectedIndex = -1;
        break;
    }
  }

  function scrollToSelected() {
    if (selectedIndex >= 0) {
      const rowIndex = Math.floor(selectedIndex / columns);
      $virtualizer.scrollToIndex(rowIndex, { align: "auto" });
    }
  }

  function handleImageClick(index: number) {
    selectedIndex = index;
    gridEl?.focus();
    console.log(filterd[index]);
  }

  function onImageSelect(image: Image) {
    console.log("selected");
  }

  async function selectWallpaper(wallpaper: Image) {
    console.log("Selected wallpaper:", wallpaper);
    await invoke("select_wallpaper", { image: wallpaper });
    // Add your wallpaper selection logic here
  }

  let virtualizer = $derived(
    createVirtualizer<HTMLDivElement, HTMLDivElement>({
      count: Math.ceil(filterd.length / columns),
      getScrollElement: () => virtualListEl ?? null,
      estimateSize: () => 158,
      overscan: 5,
    }),
  );

  const items = $derived($virtualizer.getVirtualItems());

  $effect(() => {
    const newRowCount = Math.ceil(filterd.length / columns);

    if (newRowCount !== prevRowCount)
      // Recreate virtualizer when filtered items change
      virtualizer = createVirtualizer<HTMLDivElement, HTMLDivElement>({
        count: newRowCount,
        getScrollElement: () => virtualListEl!,
        estimateSize: () => 158,
        overscan: 5,
      });
  });

  $effect(() => {
    if (virtualItemEls.length) {
      virtualItemEls.forEach((el) => {
        if (el) $virtualizer.measureElement(el);
      });
    }
  });

  onMount(async () => {
    // Focus the input element after the component has mounted
    if (virtualListEl) {
      virtualListEl.focus();
    }

    const unlisten = await listen<Image>(
      "new-image",
      (event: TauriEvent<Image>) => {
        wallpapers = [...wallpapers, event.payload];
      },
    );

    invoke("load_images");
  });
</script>

<main class="w-full p-5 bg-gray-700">
  <!-- <div class="flex items-center gap-2"> -->
  <!-- <Input -->
  <!--   type="text" -->
  <!--   placeholder="Search Wallpapers" -->
  <!--   oninput={onSearch} -->
  <!--   class="focus:border-amber-600 focus:shadow-amber-600 focus-visible:shadow-amber-600 ring-0 shadow-0 border-2 border-solid border-amber-950" -->
  <!--   tabindex={1} -->
  <!-- /> -->
  <!-- </div> -->

  <div
    class="overflow-y-auto mt-2 focus:ring-0 focus:border-0 focus:outline-0"
    style="height:560px;will-change: transform;"
    bind:this={virtualListEl}
    onkeydown={handleKeyDown}
    tabindex="0"
    role="button"
  >
    <div
      style="position: relative; height: {$virtualizer.getTotalSize()}px; width: 100%;"
    >
      <div
        class="grid grid-cols-3 gap-2"
        style="position: absolute; top: 0; left: 0; width: 100%; transform: translateY({items[0]
          ? items[0].start
          : 0}px);"
      >
        {#each items as row (row.index)}
          {#each Array(columns) as _, col}
            {@const itemIndex = row.index * columns + col}
            {#if filterd[itemIndex]}
              {@const img = filterd[itemIndex]}
              <div
                class="group cursor-pointer border-2 border-solid rounded-sm overflow-hidden relative hover:border-amber-500 hover:shadow-xl transition-all outline-0 ring-0"
                class:border-amber-950={selectedIndex !== itemIndex}
                class:border-amber-600={selectedIndex === itemIndex}
                class:border-3={selectedIndex === itemIndex}
                transition:fade
                style="height:150px"
                bind:this={virtualItemEls[row.index]}
                data-index={row.index}
                onclick={() => handleImageClick(itemIndex)}
                onkeypress={(e) =>
                  e.key === "Enter" && handleImageClick(itemIndex)}
                role="button"
                tabindex="-2"
              >
                <img
                  src={convertFileSrc(img.path)}
                  alt={img.name}
                  class="w-full h-full rounded-xs max-h-full"
                  loading="lazy"
                />
                <div
                  class="text-white z-50 absolute -bottom-12 p-1 bg-black w-full transition-all delay-50 duration-300 ease-in-out group-hover:bottom-0 opacity-60"
                >
                  {img.name}
                </div>
              </div>
            {/if}
          {/each}
        {/each}
      </div>
    </div>
  </div>
</main>

<script lang="ts">
  import "../app.css";
  import { convertFileSrc, invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import { type Event as TauriEvent, listen } from "@tauri-apps/api/event";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  const columns = 3;
  let timeout: number;

  type Image = { name: string; img_path: string; thumbnail_path: string };

  let wallpapers = $state<Image[]>([]);
  let scrollContainer = $state<HTMLDivElement>();
  let selectedIndex = $state<number>(0);

  let search = $state<string>("");
  let filterd = $derived(
    wallpapers.filter((v) => v.name.toLowerCase().includes(search)),
  );

  function onSearch(e: Event) {
    const target = e.target as HTMLInputElement;
    const v = target.value;
    clearTimeout(timeout);
    timeout = setTimeout(() => (search = v.toLowerCase()), 300);
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
        if (selectedIndex >= 0 && filterd[selectedIndex]) {
          selectWallpaper(filterd[selectedIndex]);
        }
        break;
    }
  }

  function scrollToSelected() {
    if (selectedIndex >= 0 && scrollContainer) {
      const element = scrollContainer.querySelector(
        `[data-index="${selectedIndex}"]`,
      ) as HTMLElement;
      element?.scrollIntoView({ behavior: "smooth", block: "nearest" });
    }
  }

  function handleImageClick(index: number) {
    selectedIndex = index;
    scrollContainer?.focus();
    selectWallpaper(filterd[index]);
  }

  function selectWallpaper(wallpaper: Image) {
    invoke("select_wallpaper", { image: wallpaper }).catch((err) => {
      console.error("Failed to set wallpaper:", err);
    });
  }

  let unlisten: (() => void) | undefined;
  let handleGlobalKeyDown: ((e: KeyboardEvent) => void) | undefined;

  onMount(async () => {
    // Simple batching for better performance
    let batch: Image[] = [];
    let batchTimeout: number | null = null;

    unlisten = await listen<Image>("new-image", (event: TauriEvent<Image>) => {
      batch.push(event.payload);

      if (batchTimeout !== null) {
        clearTimeout(batchTimeout);
      }

      batchTimeout = setTimeout(() => {
        wallpapers = [...wallpapers, ...batch];
        batch = [];
        batchTimeout = null;
      }, 50);
    });

    // Global ESC key listener to close the app
    handleGlobalKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.preventDefault();
        getCurrentWindow().close();
      }
    };

    window.addEventListener("keydown", handleGlobalKeyDown);
    document.addEventListener("contextmenu", (e) => {
      e.preventDefault();
    });

    invoke("load_images");

    // Focus container after a short delay to ensure DOM is ready
    setTimeout(() => {
      scrollContainer?.focus();
    }, 100);
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    if (handleGlobalKeyDown) {
      window.removeEventListener("keydown", handleGlobalKeyDown);
    }
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

  <!-- svelte-ignore a11y_autofocus -->
  <div
    class="overflow-y-auto mt-2 focus:outline-none"
    style="scroll-behavior:smooth;height: calc( 100vh - 50px)"
    bind:this={scrollContainer}
    onkeydown={handleKeyDown}
    tabindex={0}
    autofocus
    role="button"
    aria-label="Wallpaper grid"
  >
    <div class="grid grid-cols-3 gap-2" style="contain:layout style paint;">
      {#each filterd as img, index (img.img_path)}
        <div
          class="group cursor-pointer border-2 border-solid rounded-sm overflow-hidden relative hover:border-amber-500 hover:shadow-xl transition-[border-color,box-shadow] outline-0 ring-0"
          class:border-amber-950={selectedIndex !== index}
          class:border-amber-600={selectedIndex === index}
          class:border-3={selectedIndex === index}
          style="height:150px; contain:layout style paint;"
          data-index={index}
          onclick={() => handleImageClick(index)}
          onkeypress={(e) => e.key === "Enter" && handleImageClick(index)}
          role="button"
          tabindex={-1}
        >
          <img
            src={convertFileSrc(img.thumbnail_path)}
            alt={img.name}
            class="w-full h-full rounded-xs object-cover"
            width="320"
            height="150"
          />
          <div
            class="text-white z-50 absolute -bottom-12 p-1 bg-black w-full transition-[bottom] duration-300 ease-in-out group-hover:bottom-0 opacity-60"
          >
            {img.name}
          </div>
        </div>
      {/each}
    </div>
  </div>
</main>

<script lang="ts">
  import { emit, listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import Titlebar from "../lib/titlebar.svelte";
  import Toggle from "../lib/toggle.svelte";
  import ProgressBar from "../lib/progress-bar.svelte";
  import Dropdown from "../lib/dropdown.svelte";
  import Manga from "../lib/manga.svelte";

  let toggled = false;

  let totalSize = 0;

  var list: any[] = [];

  let disabled = false;

  let sort = "Default";

  let mangaFiltered: any[];

  let mangaSortedAndFiltered: any[];

  $: listLength = list.length;

  $: {
    mangaFiltered = list.filter((x) => {
      if (!toggled) {
        return x["remaining_chapters"] > 0;
      } else {
        return true;
      }
    });

    mangaSortedAndFiltered = mangaFiltered.toSorted((a, b) => {
      if (sort == "Index") {
        return a["index"] - b["index"];
      } else if (sort == "Remaining") {
        return a["remaining_chapters"] - b["remaining_chapters"];
      }
      return 0;
    });
  }

  onMount(async () => {
    await listen("scan_data", (event) => {
      if (event.payload == "done") {
        disabled = false;
      } else {
        const data = JSON.parse(JSON.stringify(event.payload));
        if (data["size"]) {
          disabled = true;
          totalSize = data["size"];
          list = [];
        } else if (data["name"]) {
          list = [...list, data];
        }
      }
    });
  });

  function eventEmit() {
    if (!disabled) {
      emit("start_scan", {
        the_message: "read_manga",
      });
    }
  }
</script>

<div data-tauri-drag-region class="app-bar">
  <Titlebar />
</div>

<div class="app">
  <div class="top-progress-bar-area">
    <ProgressBar value="{listLength}" max="{totalSize}" />
  </div>

  <div class="main-area">
    <div class="side-bar">
      <div style="width: 100%;">
        <Dropdown bind:sort />
        <Toggle bind:toggled />
      </div>

      <button
        class="fetch-button font-style {disabled
          ? 'fetch-button-disabled'
          : 'fetch-button-enabled'}"
        on:click="{eventEmit}"
      >
        Fetch
      </button>
    </div>
    <div class="main-scrollbar manga-list">
      {#key toggled}
        {#key sort}
          {#each mangaSortedAndFiltered as manga}
            <Manga propValue="{manga}"></Manga>
          {/each}
        {/key}
      {/key}
    </div>
  </div>
</div>

<style>
  .app-bar {
    width: 100%;
    height: 30px;
  }
  .app {
    width: 100%;
    display: flex;
    flex-direction: column;
  }

  .main-area {
    display: flex;
    height: calc(100vh - 90px);
  }

  .side-bar {
    height: auto;
    width: 220px;
    display: flex;
    justify-content: space-between;
    align-items: end;
    flex-direction: column;
    padding: 3px;
    position: relative;
  }

  .manga-list {
    overflow: auto;
    width: 90%;
    display: flex;
    flex-wrap: wrap;
    align-content: baseline;
  }

  .fetch-button {
    color: var(--background-color);
    width: 100%;
    border: none;
    height: 60px;
  }

  .fetch-button-enabled {
    background: var(--primary-color);
    cursor: pointer;
  }

  .fetch-button-disabled {
    background: var(--primary-disabled-color);
    cursor: wait;
  }
</style>

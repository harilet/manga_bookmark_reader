<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import Titlebar from "../lib/titlebar.svelte";
  import Toggle from "../lib/toggle.svelte";
  import ProgressBar from "../lib/progress-bar.svelte";
  import Dropdown from "../lib/dropdown.svelte";
  import Manga from "../lib/manga.svelte";
  import Anime from "./anime.svelte";
  import Search from "$lib/search.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let showAllMangas = false;

  let totalSize = 0;

  var list: any[] = [];

  let disabled = false;

  let sort = "Default";

  let mangaFiltered: any[];

  let mangaSortedAndFiltered: any[];
  let mangaFinal: any[];

  $: listLength = list.length;

  $: finalLength = mangaFinal.length;
  $: modeSelected = "manga";
  $: searchText = "";

  $: {
    mangaFiltered = list.filter((x) => {
      if (x["error"]) {
        return true;
      }
      if (showAllMangas) {
        return true;
      }
      return x["remaining_chapters"] > 0;
    });

    mangaSortedAndFiltered = mangaFiltered.toSorted((a, b) => {
      if (sort == "Index") {
        return a["index"] - b["index"];
      } else if (sort == "Remaining") {
        return a["remaining_chapters"] - b["remaining_chapters"];
      }
      return 0;
    });

    mangaFinal = mangaSortedAndFiltered.filter((x) => {
      let url: string = x["url"];
      let name: string = x["name"];
      if (url.search(searchText) != -1) {
        return true;
      } else if (name.search(searchText) != -1) {
        return true;
      }
      return false;
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
          sort = "Default";
        } else if (data["name"]) {
          list = [...list, data];
        }
      }
    });
  });

  function eventEmit() {
    if (!disabled) {
      invoke("get_manga");
    }
  }
</script>

<div data-tauri-drag-region class="app-bar">
  <Titlebar bind:selectedTab={modeSelected} />
</div>

{#if modeSelected == "manga"}
  <div class="app">
    <div class="top-progress-bar-area">
      <ProgressBar
        value={listLength}
        max={totalSize}
        listCount={finalLength}
      />
    </div>

    <div class="main-area">
      <div class="side-bar">
        <div style="width: 150px; margin-left: 20px;">
          <Dropdown bind:sort />
        </div>
        <div style="width: 150px;margin-left: 20px;">
          <Toggle bind:toggled={showAllMangas} />
        </div>

        <div>
          <Search bind:searchText />
        </div>

        <button
          data-augmented-ui="
          bl-2-clip-x tl-clip
        "
          class="fetch-button font-style {disabled
            ? 'fetch-button-disabled'
            : 'fetch-button-enabled'}"
          on:click={eventEmit}
        >
          Fetch
        </button>
      </div>
      <div class="main-scrollbar manga-list">
        {#key showAllMangas}
          {#key sort}
            {#each mangaFinal as manga}
              <Manga propValue={manga}></Manga>
            {/each}
          {/key}
        {/key}
      </div>
    </div>
  </div>
{:else if modeSelected == "anime"}
  <Anime />
{/if}

<style>
  .top-progress-bar-area {
    background: var(--accent);
  }
  .app-bar {
    width: 100%;
    height: 35px;
  }
  .app {
    width: 100%;
    display: flex;
    flex-direction: column;
  }

  .main-area {
    display: flex;
    flex-direction: column;
    height: calc(100vh - 90px);
  }

  .side-bar {
    height: auto;
    width: 100%;
    display: flex;
    align-items: center;
    flex-direction: row;
    position: relative;
    border-top: var(--border);
    background: var(--accent);
  }

  .manga-list {
    overflow: auto;
    width: 100%;
    display: flex;
    flex-wrap: wrap;
    align-content: baseline;
  }

  .fetch-button {
    color: var(--background-color);
    width: 200px;
    border: none;
    height: 60px;
    margin-left: auto;
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

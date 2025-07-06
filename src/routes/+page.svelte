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
  import Random from "../lib/icon/random.svelte";
  import dataStore from "$lib/data_store";
  import Refresh from "$lib/icon/refresh.svelte";
  import DeleteHistory from "$lib/icon/delete_history.svelte";

  let showAllMangas = false;

  let totalSize = 0;

  var list: any[] = [];

  let disabled = false;

  let sort = "Default";

  let mangaFiltered: any[];

  let mangaSortedAndFiltered: any[];
  let mangaFinal: any[];
  let mangaSample: any[] = [];
  let shuffleListDialog: HTMLDialogElement;

  $: shuffleSample = false;
  $: sampleCount = 5;

  $: listLength = list.length;

  $: finalLength = mangaFinal.length;
  $: modeSelected = "manga";
  $: searchText = "";
  $: savedUrlListLength = 0;

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
    dataStore.get("savedUrlList").then((data: any) => {
      if (data != null) {
        savedUrlListLength = data.length;
      }
    });

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

    shuffleListDialog.addEventListener("click", function (event) {
      var rect = shuffleListDialog.getBoundingClientRect();
      var isInDialog =
        rect.top <= event.clientY &&
        event.clientY <= rect.top + rect.height &&
        rect.left <= event.clientX &&
        event.clientX <= rect.left + rect.width;
      if (!isInDialog) {
        shuffleListDialog.close();
      }
    });
  });

  function eventEmit() {
    if (!disabled) {
      invoke("get_manga");
    }
  }

  function openRandom() {
    shuffleListDialog.showModal();
  }

  async function dataSaving() {
    mangaSample = [];
    for (let i = 0; i < sampleCount; i++) {
      let data: any[] | null = await dataStore.get("savedUrlList");

      if (data != null) {
        let shuffledArray = list.toSorted((a, b) => {
          return a["index"] - b["index"];
        });

        let savedUrlList: any[] = [...data];
        if (shuffleSample) {
          shuffledArray = shuffleArray(shuffledArray);
        }

        if (savedUrlList.length == list.length) {
          await dataStore.set("savedUrlList", []);
          savedUrlList = [];
        }

        // Loop through each URL in the list
        for (let urlIndex = 0; urlIndex < shuffledArray.length; urlIndex++) {
          let url = shuffledArray[urlIndex]["url"];

          // Check if the URL is already in the saved list
          if (!savedUrlList.includes(url)) {
            mangaSample = [...mangaSample, shuffledArray[urlIndex]];
            savedUrlList.push(url); // Add the URL to the saved list
            await dataStore.set("savedUrlList", savedUrlList);
            savedUrlListLength = savedUrlList.length;
            break;
          }
        }
      } else {
        await dataStore.set("savedUrlList", []);
      }
    }
  }

  function shuffleArray(array: any[]) {
    let returnArray = [...array];
    for (let i = returnArray.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [returnArray[i], returnArray[j]] = [returnArray[j], returnArray[i]];
    }
    return returnArray;
  }

  function clearRandomHistory() {
    dataStore.delete("savedUrlList");
  }
</script>

<dialog bind:this={shuffleListDialog} on:close class="main-scrollbar">
  <div style="display: flex;flex-direction: column;height: 100%;">
    <div
      style="display: flex;flex-direction: row; justify-content: space-between;"
    >
      <Toggle label="Shuffle" bind:toggled={shuffleSample} />
      <input
        type="text"
        placeholder="Number of threads"
        bind:value={sampleCount}
        style="text-align: center;"
      />
      <button class="btn" on:click={() => dataSaving()}> <Refresh /> </button>
      <button class="btn" on:click={clearRandomHistory}>
        <DeleteHistory />
      </button>
    </div>
    <div class=" manga-list">
      {#key mangaSample}
        {#each mangaSample as manga_item}
          <Manga propValue={manga_item}></Manga>
        {/each}
      {/key}
    </div>
  </div>
</dialog>

<div data-tauri-drag-region class="app-bar">
  <Titlebar bind:selectedTab={modeSelected} />
</div>

{#if modeSelected == "manga"}
  <div class="app">
    <div class="top-progress-bar-area">
      <ProgressBar value={listLength} max={totalSize} listCount={finalLength} />
    </div>

    <div class="main-area">
      <div class="side-bar">
        <div style="width: 150px; margin-left: 20px;">
          <Dropdown bind:sort />
        </div>
        <div style="width: 150px;margin-left: 20px;">
          <Toggle label="Show All manga" bind:toggled={showAllMangas} />
        </div>

        <div>
          <Search bind:searchText />
        </div>

        <button class="btn" on:click={openRandom}>
          <Random />
        </button>
        <div class="label">
          {savedUrlListLength}
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

  .btn {
    background: transparent;
    border: none;
    cursor: pointer;
    margin-left: 20px;
  }

  .label {
    color: var(--primary-color);
  }

  dialog {
    border-radius: 5px;
    border-width: 1px;
    transition: all 2s;
    width: 90%;
    z-index: 2;
    height: 70%;
    padding: 20px;
    background: var(--background-color);
    border: var(--border);
  }

  dialog::backdrop {
    background: linear-gradient(rgba(0, 0, 0, 0.4), rgba(0, 0, 0, 0.7));
    animation: fade-in 1s;
  }

  input[type="text"] {
    height: 20px;
    background: transparent;
    color: var(--primary-color);
    border: 1px solid var(--primary-color);
  }

  input[type="text"]:focus-visible {
    outline: none !important;
  }
</style>

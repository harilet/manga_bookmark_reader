<script lang="ts">
  import { emit, listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import Titlebar from "../lib/titlebar.svelte";
  import Toggle from "../lib/toggle.svelte";
  import ProgressBar from "../lib/progress-bar.svelte";
  import Dropdown from "../lib/dropdown.svelte";
  import Manga from "../lib/manga.svelte";
  import Anime from "./anime.svelte";
  import Search from "$lib/search.svelte";
  import List from "$lib/icon/list.svelte";
  import Input from "$lib/input.svelte";
  import dataStore from "$lib/data_store";

  let showAllMangas = false;

  let totalSize = 0;

  var list: any[] = [];

  let disabled = false;

  let sort = "Default";

  let mangaFiltered: any[];

  let mangaSortedAndFiltered: any[];
  let mangaFinal: any[];

  $: threadCount = "0";
  $: activeThread = 0;

  $: listLength = list.length;

  $: finalLength = mangaFinal.length;
  $: modeSelected = "manga";
  $: searchText = "";
  $: mangaInThread = [""];
  let mangaInThreadDialog: HTMLDialogElement;

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
    mangaInThread = mangaInThread.filter((x) => x !== "");
    let tempThreadCount = await dataStore.get("thread_count");
    if (typeof tempThreadCount === "string") {
      threadCount = tempThreadCount;
    }

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
    await listen("threads", (event) => {
      let data: string[] = String(event.payload).split("/");
      activeThread = parseInt(data[0]);
    });

    await listen("thread_update", (event) => {
      let data = String(event.payload);
      if (data[0] == "+") {
        mangaInThread = [...mangaInThread, data.substring(1)];
      } else {
        let temp = mangaInThread.filter(function (x) {
          return x != data.substring(1);
        });
        mangaInThread = [];
        mangaInThread = [...temp];
      }
    });

    mangaInThreadDialog.addEventListener("click", function (event) {
      var rect = mangaInThreadDialog.getBoundingClientRect();
      var isInDialog =
        rect.top <= event.clientY &&
        event.clientY <= rect.top + rect.height &&
        rect.left <= event.clientX &&
        event.clientX <= rect.left + rect.width;
      if (!isInDialog) {
        mangaInThreadDialog.close();
      }
    });
  });

  function eventEmit() {
    if (!disabled) {
      emit("start_scan", {
        worker_count: parseInt(threadCount),
      });
    }
  }

  function toggleDialog() {
    if (mangaInThreadDialog.open) {
      mangaInThreadDialog.close();
    } else {
      mangaInThreadDialog.showModal();
    }
  }

  async function onThreadCountChange(data: any) {
    await dataStore.set("thread_count", data.target.value);
    threadCount=data.target.value;
  }
</script>

<div data-tauri-drag-region class="app-bar">
  <Titlebar bind:selectedTab={modeSelected} />
</div>

<dialog bind:this={mangaInThreadDialog} on:close>
  <div style="border: solid 1px var(--primary-color);width: 99%;">
    {#key mangaInThread}
      {#each mangaInThread as manga_item}
        <div class="manga-in-thread-wrapper">
          <div
            data-augmented-ui="
        r-rect l-rect
      "
            class="manga-in-thread"
          >
            {#if manga_item !== ""}
              {manga_item}
            {/if}
          </div>
        </div>
      {/each}
    {/key}
  </div>
</dialog>

{#if modeSelected == "manga"}
  <div class="app">
    <div class="top-progress-bar-area">
      {#key threadCount != ""}
        <ProgressBar
          value={listLength}
          max={totalSize}
          threadValue={activeThread}
          maxThread={parseInt(threadCount)}
          {threadCount}
          listCount={finalLength}
          {onThreadCountChange}
        />
      {/key}
    </div>

    <div class="main-area">
      <div class="side-bar">
        <div>
          <button class="thread-manga-btn" on:click={toggleDialog}>
            <List />
          </button>
        </div>
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

  .manga-in-thread-wrapper {
    background: var(--primary-color);
    border: var(--border);
  }

  .manga-in-thread {
    background: var(--background-color);
    padding: 20px;
    color: var(--primary-color);
  }

  .thread-manga-btn {
    background: transparent;
    border: var(--border);
  }
</style>

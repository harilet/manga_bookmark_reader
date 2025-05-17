<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import dataStore from "./data_store";

  export let value = 0;
  export let max = 0;
  export let listCount = 0;

  $: threadCount = 0;
  $: currentThreadCount = 0;
  let progress_bar_class = "";
  let mangaInThreadDialog: HTMLDialogElement;
  $: mangaInThread = [""];

  $: {
    if (value >= max) {
      progress_bar_class = "progress_done";
    } else {
      progress_bar_class = "in_progress";
    }
  }

  onMount(async () => {
    mangaInThread = mangaInThread.filter((x) => x !== "");
    let tempThreadCount = await dataStore.get("thread_count");
    if (typeof tempThreadCount === "string") {
      threadCount = Number.parseInt(tempThreadCount);
      changeThreadCount();
    }

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

    await listen("thread_update", (event: { payload: any }) => {
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

    await listen("threads", (event) => {
      let data: string[] = String(event.payload).split("/");
      currentThreadCount = parseInt(data[0]);
    });
  });

  function showThreadCountInput() {
    if (mangaInThreadDialog.open) {
      mangaInThreadDialog.close();
    } else {
      mangaInThreadDialog.showModal();
    }
  }

  function changeThreadCount() {
    invoke("update_thread_count", {
      newThreadCount: parseInt(threadCount + ""),
    });
  }

  async function onThreadCountChange(data: any) {
    let dataValue = data.target.value;
    if (dataValue !== "" && dataValue != "0") {
      dataValue = parseInt(dataValue);
      await dataStore.set("thread_count", dataValue + "");
      threadCount = dataValue;
      changeThreadCount();
      mangaInThreadDialog.close();
    }
  }
</script>

<dialog bind:this={mangaInThreadDialog} on:close>
  <input
    type="text"
    placeholder="Number of threads"
    value={threadCount}
    on:change={onThreadCountChange}
    style="text-align: center;"
  />
  {#key mangaInThread}
    {#each mangaInThread as manga_item}
      <div style="border: solid 1px var(--primary-color);width: 99%;">
        <div class="manga-in-thread-wrapper">
          <div data-augmented-ui="r-rect l-rect" class="manga-in-thread">
            {#if manga_item !== ""}
              {manga_item}
            {/if}
          </div>
        </div>
      </div>
    {/each}
  {/key}
</dialog>

<div class="progress-area-wrapper">
  <div
    data-augmented-ui="bl-clip t-clip-x br-clip border"
    class="progress-area"
  >
    <progress
      data-augmented-ui="bl-clip t-clip-x br-clip"
      class="progress {progress_bar_class}"
      {value}
      {max}
    >
    </progress>
  </div>
</div>

<div class="progress-test">
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div style="display: flex;cursor: pointer;" on:click={showThreadCountInput}>
    {#key threadCount}
      {#each [...Array(threadCount).keys()] as i (i)}
        <div
          class="thread-box"
          style="--fill: {currentThreadCount <= i
            ? 'transparent'
            : 'var(--primary-color)'};"
        >
          <span></span>
        </div>
      {/each}
    {/key}
  </div>

  <div>
    {listCount} : {value} of {max}
  </div>
</div>

<style>
  .thread-box {
    flex: 1;
    transition: opacity 0.3s ease;
    background-color: var(--fill);
    border: var(--border);
    width: 10px;
    height: 10px;
    margin: 0px 1px;
  }

  .progress-area-wrapper {
    padding: 0px;
  }

  .progress-area {
    display: flex;
    padding: 4px 2px;
    justify-content: space-evenly;
    --aug-border-all: 1px;
    --aug-border-bg: var(--primary-color);
  }

  .progress-test {
    padding: 5px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    color: var(--primary-color);
  }

  .progress {
    width: 100%;
    border-radius: 0px;
    height: 20px;
  }

  .in_progress::-webkit-progress-value {
    background: repeating-linear-gradient(
      90deg,
      var(--in-progress-color),
      var(--in-progress-color) 1px,
      var(--background-color) 1px,
      var(--background-color) 3px
    );
  }
  .progress_done::-webkit-progress-value {
    background: repeating-linear-gradient(
      90deg,
      var(--done-color),
      var(--done-color) 1px,
      var(--background-color) 1px,
      var(--background-color) 3px
    );
  }
  .progress[value]::-webkit-progress-bar {
    background-color: transparent;
  }

  .thread-progress {
    background: transparent;
    border: var(--border);
  }

  .thread-progress[value]::-webkit-progress-bar {
    background-color: transparent;
  }

  .thread-progress::-webkit-progress-value {
    background: var(--done-color);
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
    border: none;
  }

  input[type="text"]:focus-visible {
    outline: none !important;
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
</style>

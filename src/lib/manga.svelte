<script lang="ts">
  import { open } from "@tauri-apps/plugin-shell";
  import dataStore from "./data_store";

  export let propValue: any;

  let borderClass = "";
  let wrapperClass = "";
  let reTrigger = false;

  $: {
    reTrigger = !reTrigger;
    setClasses();
  }

  async function setClasses() {
    if (await dataStore.get(getUrlWithoutChapter())) {
      borderClass = "warning-class";
      wrapperClass = "warning-class-wrapper";
    } else {
      borderClass = "normal-class";
      wrapperClass = "normal-class-wrapper";
    }
  }

  function openUrl() {
    open(propValue["url"]);
  }

  async function setChapterWarning(event: any) {
    event.stopPropagation();

    let url = getUrlWithoutChapter();
    if (await dataStore.get(url)) {
      await dataStore.delete(url);
    } else {
      await dataStore.set(url, true);
    }
    await dataStore.save();
    reTrigger = !reTrigger;
  }

  function getUrlWithoutChapter() {
    return propValue["url"].split("/").slice(0, -1).join("/");
  }
</script>

<div class="{wrapperClass} wrapper-class">
  <button
    data-augmented-ui="tr-clip-x tl-clip bl-clip-x"
    on:click="{openUrl}"
    class="main-manga-area {borderClass} font-style"
  >
    <div class="index-date">
      <div>
        {propValue["index"]}
      </div>
      <div class="date-time">
        {propValue["date_time"]}
      </div>
    </div>
    <div class="name">
      {propValue["name"]}
    </div>
    <div class="remaining-chapter">
      <div class="remaining-chapter-text">
        {propValue["remaining_chapters"]}
      </div>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div
        on:click="{setChapterWarning}"
        data-augmented-ui="tl-clip"
        class="warning-button"
      >
        &nbsp;
      </div>
    </div>
  </button>
</div>

<style>
  .main-manga-area {
    border-radius: 5px;
    cursor: pointer;
    padding: 0px;
    padding-left: 5px;
    position: relative;
    width: 100%;
    color: var(--cds-text-01);
    border: none;
    background: var(--background-color);
    text-align: start;
    display: flex;
    flex-direction: column;
    align-items: center;
    --aug-border-all: 2px;
    height: 100%;
    justify-content: space-between;
  }

  .wrapper-class {
    padding: 2px;
    flex: auto;
  }

  .warning-class-wrapper {
    background: var(--warning-color);
  }

  .normal-class-wrapper {
    background: var(--primary-color);
    color: var(--primary-color);
  }

  .warning-class {
    color: var(--warning-color);
  }

  .normal-class {
    --aug-border-bg: var(--primary-color);
  }

  .index-date {
    display: flex;
    justify-content: space-between;
    margin-bottom: 10px;
    padding-right: 5px;
    margin-top: 20px;
    color: gray;
    width: -webkit-fill-available;
  }

  .date-time {
    margin-top: 5px;
  }

  .name {
    margin-bottom: 10px;
    font-weight: 600;
    font-size: 105%;
  }

  .remaining-chapter {
    display: flex;
    justify-content: space-between;
    align-items: end;
    font-weight: 600;
    width: 100%;
  }

  .remaining-chapter-text {
    margin-bottom: 20px;
  }

  .warning-button {
    width: 40px;
    --aug-all-height: 5vh;
    background: repeating-linear-gradient(
      90deg,
      var(--warning-color),
      var(--warning-color) 1px,
      transparent 1px,
      transparent 3px
    );
  }
</style>

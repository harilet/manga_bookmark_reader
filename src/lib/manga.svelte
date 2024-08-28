<script lang="ts">
  import { open } from "@tauri-apps/plugin-shell";
  import dataStore from "./data_store";

  export let propValue: any;

  let borderClass = "";
  let reTrigger = false;

  $: {
    reTrigger = !reTrigger;
    setClasses();
  }

  async function setClasses() {
    if (await dataStore.get(getUrlWithoutChapter())) {
      borderClass = "warning-class";
    } else {
      borderClass = "normal-class";
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

  <button
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
        class="warning-button"
      >
        &nbsp;
      </div>
    </div>
  </button>

<style>
  .main-manga-area {
    border-radius: 5px;
    cursor: pointer;
    padding: 0px;
    padding-left: 5px;
    position: relative;
    background: transparent;
    text-align: start;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    flex-grow: 1;
    border-radius: 0px;
  }

  .wrapper-class {
    padding: 2px;
    flex: auto;
  }

  .warning-class {
    color: var(--warning-color);
    border: solid 1px var(--warning-color);
  }

  .normal-class {
    color: var(--primary-color);
    border: solid 1px var(--primary-color);
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
    background: repeating-linear-gradient(
      90deg,
      var(--warning-color),
      var(--warning-color) 1px,
      transparent 1px,
      transparent 3px
    );
  }
</style>

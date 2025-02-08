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

<button on:click={openUrl} class="button-wrappper">
  <div
    data-augmented-ui="
      tr-2-clip-x tl-2-clip-x
    "
    class="main-manga-area {borderClass} font-style"
  >
    <div class="index-date">
      <div class="name">
        {propValue["name"]}
      </div>
      <div class="date-time">
        {propValue["date_time"]}
      </div>
    </div>

    <div class="remaining-chapter">
      <div class="remaining-chapter-text">
        {propValue["remaining_chapters"]}
      </div>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div on:click={setChapterWarning} class="warning-button">&nbsp;</div>
    </div>
  </div>
</button>

<style>
  .button-wrappper {
    flex-grow: 1;
    padding: 0px;
    border: none;
  }
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
    border-radius: 0px;
  }

  .wrapper-class {
    padding: 2px;
    flex: auto;
  }

  .warning-class {
    color: var(--primary-color);
    border: solid 1px rgb(255 255 255 / 0.1);
    text-shadow:
      2px 2px 0 #000000,
      2px -2px 0 #000000,
      -2px 2px 0 #000000,
      -2px -2px 0 #000000,
      2px 0px 0 #000000,
      0px 2px 0 #000000,
      -2px 0px 0 #000000,
      0px -2px 0 #000000;
    background: repeating-linear-gradient(
      90deg,
      var(--primary-color),
      var(--primary-color) 1px,
      var(--background-color) 1px,
      var(--background-color) 3px
    );
  }

  .normal-class {
    color: var(--primary-color);
    border: solid 1px rgb(255 255 255 / 0.1);
    background: var(--background-color);
  }

  .index-date {
    display: flex;
    justify-content: space-between;
    margin-bottom: 5px;
    padding-right: 5px;
    margin-top: 20px;
    width: -webkit-fill-available;
  }

  .date-time {
    margin-top: 5px;
    color: gray;
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
    margin-bottom: 10px;
    color: gray;
  }

  .normal-class .warning-button {
    width: 40px;
    background: repeating-linear-gradient(
      90deg,
      var(--primary-color),
      var(--primary-color) 1px,
      var(--background-color) 1px,
      var(--background-color) 3px
    );
  }

  .warning-class .warning-button {
    width: 40px;
    background: var(--background-color);
  }
</style>

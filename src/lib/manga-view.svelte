<script lang="ts">
  import dataStore from "./data_store";
  import { open } from "@tauri-apps/plugin-shell";

  export let mangaData: any;

  function getMangaHost() {
    let url = mangaData["url"];
    let host = url.split("/")[2];
    host = host.replace("www.", "");
    host = host.split(".")[0];
    return host;
  }

  function getStatusClass() {
    if (mangaData["error"]) {
      return "error";
    }
    return "synced";
  }

  function openUrl() {
    open(mangaData["url"]);
  }

  async function setClasses() {
    if (await dataStore.get(getUrlWithoutChapter())) {
      return true;
    } else {
      return false;
    }
  }

  function getUrlWithoutChapter() {
    return mangaData["url"].split("/").slice(0, -1).join("/");
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="card {getStatusClass()}" on:click={openUrl}>
  <div class="card-top">
    <div class="hex">{mangaData["index"]}</div>
    <div class="title">{mangaData["name"]}</div>
  </div>
  <div class="source-row">
    <span class="pill">{getMangaHost()}</span>
    <span class="checks">{getStatusClass()} <b>1</b>×</span>
  </div>
  <div class="meter">
    <span class="tick on {getStatusClass()}"></span><span
      class="tick on {getStatusClass()}"
    ></span><span class="tick on {getStatusClass()}"></span><span
      class="tick on {getStatusClass()}"
    ></span><span class="tick on {getStatusClass()}"></span><span
      class="tick on {getStatusClass()}"
    ></span><span class="tick on {getStatusClass()}"></span><span
      class="tick on {getStatusClass()}"
    ></span><span class="tick on {getStatusClass()}"></span><span
      class="tick on {getStatusClass()}"
    ></span><span class="tick on {getStatusClass()}"></span><span
      class="tick on {getStatusClass()}"
    ></span><span class="tick on {getStatusClass()}"></span><span
      class="tick on {getStatusClass()}"
    ></span><span class="tick on {getStatusClass()}"></span><span
      class="tick on {getStatusClass()}"
    ></span>
  </div>
  <div class="meter-row">
    <span>sync</span><span>
      {#if mangaData["total_chapters"] > 0}
        {Math.round(
          ((mangaData["total_chapters"] - mangaData["remaining_chapters"]) /
            mangaData["total_chapters"]) *
            100,
        )}%
      {/if}</span
    >
  </div>
  <div class="foot">
    <span class="date">{mangaData["date_time"]}</span>
    <span class="status {getStatusClass()}"
      ><span class="dot"></span>{getStatusClass()}</span
    >
  </div>
</div>

<style>
  .card {
    background: var(--panel);
    border: 1px solid var(--border-dim);
    border-left: 3px solid var(--ink-dim);
    border-radius: 0 8px 8px 0;
    padding: 16px 18px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    transition:
      border-color 0.15s ease,
      background 0.15s ease;
    flex-grow: 1;
    margin: 10px;
  }
  .card:hover {
    border-color: var(--border-mid);
    background: var(--panel-soft);
  }
  .card.synced {
    border-left-color: var(--signal);
  }
  .card.syncing {
    border-left-color: var(--amber);
  }
  .card.error {
    border-left-color: var(--fault);
  }
  .card.queued {
    border-left-color: var(--ink-muted);
  }

  .card-top {
    display: flex;
    gap: 10px;
    align-items: flex-start;
  }

  .hex {
    flex: none;
    width: 30px;
    height: 30px;
    clip-path: polygon(50% 0%, 93% 25%, 93% 75%, 50% 100%, 7% 75%, 7% 25%);
    background: var(--panel-soft);
    border: 1px solid var(--border-mid);
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--mono);
    font-size: 11px;
    color: var(--ink-muted);
  }
  .title {
    flex: 1;
    min-width: 0;
    font-family: var(--mono);
    font-size: 13.5px;
    line-height: 1.45;
    font-weight: 500;
    color: var(--ink);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .source-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-family: var(--mono);
    font-size: 11px;
  }
  .pill {
    background: var(--panel-soft);
    border: 1px solid var(--border-dim);
    color: var(--ink-muted);
    padding: 3px 9px;
    border-radius: 10px;
  }
  .checks {
    color: var(--ink-dim);
  }
  .checks b {
    color: var(--ink-muted);
    font-weight: 500;
  }

  .meter {
    display: flex;
    gap: 2px;
    height: 9px;
  }
  .tick {
    flex: 1;
    background: var(--ink-dim);
    border-radius: 1px;
  }
  .tick.on.synced {
    background: var(--signal);
  }
  .tick.on.syncing {
    background: var(--amber);
  }
  .tick.on.error {
    background: var(--fault);
  }
  .tick.on.queued {
    background: var(--ink-muted);
  }
  .meter-row {
    display: flex;
    justify-content: space-between;
    font-family: var(--mono);
    font-size: 10.5px;
    color: var(--ink-muted);
  }

  .foot {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 10px;
    border-top: 1px solid var(--border-dim);
    font-family: var(--mono);
    font-size: 11px;
  }
  .date {
    color: var(--ink-dim);
  }
  .status {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex: none;
  }
  .status.synced {
    color: var(--signal);
  }
  .status.synced .dot {
    background: var(--signal);
  }
  .status.syncing {
    color: var(--amber);
  }
  .status.syncing .dot {
    background: var(--amber);
    animation: pulse 1.6s ease-in-out infinite;
  }
  .status.error {
    color: var(--fault);
  }
  .status.error .dot {
    background: var(--fault);
  }
  .status.queued {
    color: var(--ink-muted);
  }
  .status.queued .dot {
    background: var(--ink-muted);
  }
</style>

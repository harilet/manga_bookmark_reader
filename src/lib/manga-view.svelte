<script lang="ts">
  import { onMount } from "svelte";
  import dataStore from "./data_store";
  import { open } from "@tauri-apps/plugin-shell";

  export let mangaData: any;

  $: warning = false;

  onMount(() => {
    setClasses();
  });

  function getMangaHost() {
    let url = mangaData["url"];
    let host = url.split("/")[2];
    host = host.replace("www.", "");
    host = host.split(".")[0];
    return host;
  }

  function getStatusClass() {
    if(mangaData["remaining_chapters"] <= 0) {
      return "queued";
    }
    if (mangaData["error"]) {
      return "error";
    }
    if (warning) {
      return "warning";
    }
    return "synced";
  }

  function openUrl() {
    open(mangaData["url"]);
  }

  async function setClasses() {
    if (await dataStore.get(getUrlWithoutChapter())) {
      warning = true;
    } else {
      warning = false;
    }
  }

  function getUrlWithoutChapter() {
    return mangaData["url"].split("/").slice(0, -1).join("/");
  }

  function progress() {
    return (
      (mangaData["total_chapters"] - mangaData["remaining_chapters"]) /
      mangaData["total_chapters"]
    );
  }

  async function setWarning() {
    if (await dataStore.get(getUrlWithoutChapter())) {
      dataStore.delete(getUrlWithoutChapter());
      warning = false;
    } else {
      dataStore.set(getUrlWithoutChapter(), 1);
      warning = true;
    }
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
{#key warning}
  <div class="card {getStatusClass()}" on:click={openUrl}>
    <div class="card-top">
      <div class="hex">{mangaData["index"]}</div>
      <div class="title {getStatusClass()}">
        {mangaData["name"]}
      </div>
      <div>
        <button class="btn" on:click|preventDefault|stopPropagation={setWarning}
        ></button>
      </div>
    </div>
    <div class="source-row">
      <span class="pill">{getMangaHost()}</span>
    </div>
    <div class="meter">
      {#if mangaData["total_chapters"] == 1}
        <span class="tick {getStatusClass()}"> </span>
      {:else}
        {#each { length: 101 } as _, i}
          <span
            class="tick {i < progress() * 100 ? 'on' : ''} {getStatusClass()}"
          >
          </span>
        {/each}
      {/if}
    </div>
    <div class="meter-row">
      <span>
        {#if mangaData["total_chapters"] > 0}
          {Math.round(progress() * 100)}%
        {/if}
      </span>

      <span>
        {mangaData["remaining_chapters"]}/{mangaData["total_chapters"]}
      </span>
    </div>
    <div class="foot">
      <span class="date">{mangaData["date_time"]}</span>
      <span class="status {getStatusClass()}"
        ><span class="dot"></span>{getStatusClass()}</span
      >
    </div>
  </div>
{/key}

<style>
  .card {
    background: var(--void);
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
    color: var(--ink);
  }
  .card.warning {
    border-left-color: var(--amber);
    color: var(--amber);
  }
  .card.error {
    border-left-color: var(--fault);
    color: var(--fault);
  }
  .card.queued {
    border-left-color: var(--ink-muted);
    color: var(--ink-muted);
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
    /* color: var(--ink-muted); */
  }
  .title {
    flex: 1;
    min-width: 0;
    font-family: var(--mono);
    font-size: 13.5px;
    line-height: 1.45;
    font-weight: 500;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .title.synced {
    color: var(--ink);
  }

  .title.warning {
    color: var(--amber);
  }
  .title.error {
    color: var(--fault);
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
    padding: 3px 9px;
    border-radius: 10px;
  }
  .checks {
    color: var(--ink-dim);
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
  .tick.on.warning {
    background: var(--amber);
  }
  .tick.error {
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

  .btn {
    background: var(--signal-soft);
    border: 1px solid var(--signal);
    color: var(--signal);
    font-family: var(--mono);
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.04em;
    padding: 9px 16px;
    border-radius: 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: background 0.15s ease;
  }
</style>

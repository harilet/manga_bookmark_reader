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

  function progressPct() {
    if (!mangaData["total_chapters"]) return 0;
    return Math.min(100, Math.max(0, Math.round(progress() * 100)));
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
        <button
          class="btn {warning ? 'on' : ''}"
          aria-label="toggle warning"
          title="toggle warning"
          on:click|preventDefault|stopPropagation={setWarning}>!</button
        >
      </div>
    </div>
    <div class="source-row">
      <span class="pill">{getMangaHost()}</span>
    </div>
    <div class="bar {getStatusClass()}">
      <span class="fill" style="width: {progressPct()}%"></span>
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
    position: relative;
    background: var(--panel);
    border: 1px solid var(--border-dim);
    border-radius: 6px;
    padding: 16px 18px;
    display: flex;
    flex-direction: column;
    gap: 14px;
    cursor: pointer;
    overflow: hidden;
    transition:
      border-color 0.15s ease,
      background 0.15s ease;
  }
  .card::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    width: 3px;
    height: 100%;
    background: var(--ink-dim);
    transition: background 0.15s ease;
  }
  .card:hover {
    border-color: var(--border-mid);
    background: var(--panel-soft);
  }
  .card.synced {
    color: var(--ink);
  }
  .card.synced::before {
    background: var(--signal);
  }
  .card.warning {
    color: var(--amber);
    border-color: rgba(245, 166, 35, 0.55);
    background:
      linear-gradient(rgba(245, 166, 35, 0.07), rgba(245, 166, 35, 0.07)),
      var(--panel);
  }
  .card.warning:hover {
    border-color: var(--amber);
    background:
      linear-gradient(rgba(245, 166, 35, 0.12), rgba(245, 166, 35, 0.12)),
      var(--panel);
  }
  .card.warning::before {
    background: var(--amber);
    box-shadow: 0 0 12px rgba(245, 166, 35, 0.7);
  }
  .card.error {
    color: var(--fault);
  }
  .card.error::before {
    background: var(--fault);
  }
  .card.queued {
    color: var(--ink-muted);
  }
  .card.queued::before {
    background: var(--ink-muted);
  }

  .card-top {
    display: flex;
    gap: 10px;
    align-items: flex-start;
  }

  .hex {
    flex: none;
    width: 34px;
    height: 34px;
    border-radius: 6px;
    background: var(--signal-soft);
    border: 1px solid var(--signal);
    color: var(--signal);
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--mono);
    font-size: 12px;
    font-weight: 600;
  }
  .title {
    flex: 1;
    min-width: 0;
    font-family: var(--sans);
    font-size: 15px;
    line-height: 1.35;
    font-weight: 600;
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
    font-size: 10px;
  }
  .pill {
    background: transparent;
    border: 1px solid var(--border-dim);
    padding: 3px 8px;
    border-radius: 3px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--ink-muted);
  }
  .checks {
    color: var(--ink-dim);
  }

  .bar {
    width: 100%;
    height: 8px;
    background: rgba(255, 255, 255, 0.08);
    border-radius: 999px;
    overflow: hidden;
  }
  .bar .fill {
    display: block;
    height: 100%;
    background: var(--signal);
    border-radius: inherit;
    transition: width 0.3s ease;
  }
  .bar.warning .fill {
    background: var(--amber);
  }
  .bar.error .fill {
    background: var(--fault);
  }
  .bar.queued .fill {
    background: var(--ink-muted);
  }

  .meter-row {
    display: flex;
    justify-content: space-between;
    font-family: var(--mono);
    font-size: 10.5px;
    letter-spacing: 0.06em;
    color: var(--ink-muted);
  }

  .foot {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 12px;
    border-top: 1px solid var(--border-dim);
    font-family: var(--mono);
    font-size: 10px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
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
  .status.warning {
    color: var(--amber);
  }
  .status.warning .dot {
    background: var(--amber);
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
    width: 26px;
    height: 26px;
    flex: none;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: 1px solid var(--border-dim);
    border-radius: 4px;
    color: var(--ink-dim);
    font-family: var(--mono);
    font-size: 12px;
    font-weight: 700;
    line-height: 1;
    cursor: pointer;
    transition:
      background 0.15s ease,
      border-color 0.15s ease,
      color 0.15s ease;
  }
  .btn:hover {
    border-color: var(--border-mid);
    color: var(--ink-muted);
  }
  .btn.on {
    background: var(--amber);
    border-color: var(--amber);
    color: #05070c;
  }
</style>

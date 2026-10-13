<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import Titlebar from "../lib/titlebar.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import dataStore from "$lib/data_store";
  import Cinput from "$lib/cinput.svelte";
  import ConfigFilesView from "$lib/config-files-view.svelte";
  import MangaView from "$lib/manga-view.svelte";
  import ThreadConfigView from "$lib/thread-config-view.svelte";
  import Settings from "$lib/icons/settings.svelte";

  let disabled = false;
  let totalSize = 0;
  var list: any[] = [];
  let sort = "default";
  let selected = "remaining";
  let mangaFinal: any[];
  $: searchValue = "";
  let ignoreHosts: string[] = [];
  let color = "#4f7cff";
  let showfilters = false;

  $: {
    let mangaFiltered = list.filter((x) => {
      for (let host of ignoreHosts) {
        if (x["url"].includes(host)) {
          return false;
        }
      }

      if (selected === "all") {
        return true;
      }

      if (selected !== "no_error" && x["error"]) {
        return true;
      }

      if (selected === "no_error" && x["error"]) {
        return false;
      }

      return x["remaining_chapters"] > 0;
    });

    let mangaSortedAndFiltered = mangaFiltered.toSorted((a, b) => {
      if (sort == "index") {
        return a["index"] - b["index"];
      } else if (sort == "remaining") {
        return a["remaining_chapters"] - b["remaining_chapters"];
      } else if (sort == "name") {
        return a["name"] - b["name"];
      } else if (sort == "name-length") {
        return a["name"].length - b["name"].length;
      }
      return 0;
    });

    mangaFinal = mangaSortedAndFiltered.filter((x) => {
      let url: string = x["url"];
      let name: string = x["name"];
      if (url.search(searchValue) != -1) {
        return true;
      } else if (name.search(searchValue) != -1) {
        return true;
      }
      return false;
    });
  }

  onMount(async () => {
    dataStore.get("primary-color").then((data: any) => {
      if (data != null) {
        color = data;
        changeColor();
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

  function getUniqueHosts(): Array<{ host: string; count: number }> {
    const hostCounts = new Map<string, number>();
    list.forEach((manga) => {
      try {
        const url = new URL(manga.url);
        const hostname = url.hostname;
        hostCounts.set(hostname, (hostCounts.get(hostname) || 0) + 1);
      } catch (e) {
        console.error("Invalid URL:", manga.url);
      }
    });
    return Array.from(hostCounts.entries()).map(([host, count]) => ({
      host,
      count,
    }));
  }

  function toggleIgnoreHost(host: string) {
    ignoreHosts = ignoreHosts.includes(host)
      ? ignoreHosts.filter((h) => h !== host)
      : [...ignoreHosts, host];
  }

  function generateTheme(primaryHex: string) {
    const hex = primaryHex.replace("#", "");
    const r = parseInt(hex.slice(0, 2), 16);
    const g = parseInt(hex.slice(2, 4), 16);
    const b = parseInt(hex.slice(4, 6), 16);

    const root = document.documentElement;

    root.style.setProperty(`--signal`, primaryHex);
    root.style.setProperty(`--signal-soft`, `rgba(${r},${g},${b},0.12)`);
  }

  async function changeColor() {
    await dataStore.set("primary-color", color);
    generateTheme(color);
  }

  async function onThreadCountChange(data: any) {
    searchValue = data.target.value;
  }
</script>

<div class="app-bar">
  <Titlebar>
    <div class="top-level-filter">
      <div class="field">
        <Cinput
          placeholder="search threads…"
          value={searchValue}
          onChange={onThreadCountChange}
        >
          <circle cx="11" cy="11" r="7" /><line
            x1="21"
            y1="21"
            x2="16.65"
            y2="16.65"
          />
        </Cinput>
      </div>

      <button
        class="icon-btn"
        class:active={showfilters}
        aria-label="toggle filters"
        on:click={(_) => (showfilters = !showfilters)}
      >
        <Settings />
      </button>

      <select class="filter" bind:value={selected}>
        <option value="remaining">Remaining</option>
        <option value="all">Show All</option>
        <option value="no_error">No Error</option>
      </select>

      <select class="filter" bind:value={sort}>
        <option value="default">Default</option>
        <option value="index">Index</option>
        <option value="remaining">Remaining</option>
        <option value="name">Name</option>
        <option value="name-length">Name Length</option>
      </select>

      <button class="fetch primary" on:click={eventEmit} {disabled}>
        {#if !disabled}
          <span class="pulse"></span>
        {/if}
        fetch now
      </button>
    </div>
  </Titlebar>
</div>
<div class="app">
  <div class="panel" style={showfilters ? "" : "display: none;"}>
    <div class="panel-body">
      <div class="panel-section">
        <div class="section-head">
          <span class="section-label">by source</span>
        </div>
        <div class="chips" id="chips">
          {#key mangaFinal}
            {#key ignoreHosts}
              {#each getUniqueHosts() as { host, count }}
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <span
                  class="chip {ignoreHosts.includes(host) ? 'off' : ''}"
                  on:click={() => toggleIgnoreHost(host)}
                  >{host}<span class="count">{count}</span></span
                >
              {/each}
            {/key}
          {/key}
        </div>
      </div>

      <div class="panel-divider"></div>

      <ConfigFilesView />

      <div class="panel-divider"></div>

      <ThreadConfigView />

      <div class="panel-divider"></div>
      <input
        type="color"
        id="favcolor"
        name="favcolor"
        bind:value={color}
        on:change={(_) => changeColor()}
      />
    </div>
  </div>
  <div class="overall">
    <div class="overall-head">
      <span class="overall-label">01 / library</span>
      <span class="overall-pct" id="overallPct">
        {Math.ceil((list.length / totalSize) * 100)}%
      </span>
    </div>
    <div class="overall-meter" id="overallMeter">
      {#each { length: 200 } as _, i}
        <span class="tick {i < (list.length / totalSize) * 200 ? 'on' : ''}"
        ></span>
      {/each}
    </div>
    <div class="overall-foot">
      <span>{mangaFinal.length} shown</span>
      <span>{list.length} of {totalSize} scanned</span>
    </div>
  </div>

  <div class="section-bar">
    <span class="section-label">02 / lineup</span>
    <span class="section-meta">pick a title to open</span>
  </div>

  <div class="manga-area main-scrollbar">
    {#key mangaFinal}
      {#each mangaFinal as manga}
        <MangaView mangaData={manga} />
      {/each}
    {/key}
  </div>
</div>

<style>
  .app-bar {
    width: 100%;
    flex: none;
  }

  .app {
    width: 100%;
    display: flex;
    flex-direction: column;
    height: calc(100% - 53px);
    padding: 0 26px;
    border-left: 1px solid var(--border-dim);
    border-right: 1px solid var(--border-dim);
  }

  .top-level-filter {
    display: flex;
    align-items: center;
    gap: 10px;
    height: 100%;
  }
  .top-level-filter .field {
    display: flex;
  }

  .fetch {
    font-family: var(--mono);
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    padding: 9px 16px;
    border-radius: 4px;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    background: transparent;
    border: 1px solid var(--border-mid);
    color: var(--ink);
    transition:
      background 0.15s ease,
      border-color 0.15s ease,
      color 0.15s ease;
  }
  .fetch:hover {
    background: var(--panel-soft);
    border-color: var(--ink-muted);
  }

  .fetch.primary {
    background: var(--ink);
    border-color: var(--ink);
    color: #05070c;
  }
  .fetch.primary:hover {
    background: #ffffff;
    border-color: #ffffff;
  }
  .fetch.primary .pulse {
    background: #05070c;
  }

  .icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 37px;
    height: 35px;
    flex: none;
    background: transparent;
    border: 1px solid var(--border-mid);
    border-radius: 4px;
    cursor: pointer;
    transition:
      background 0.15s ease,
      border-color 0.15s ease;
  }
  .icon-btn:hover,
  .icon-btn.active {
    background: var(--panel-soft);
    border-color: var(--ink-muted);
  }

  .pulse {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--signal);
    animation: pulse 1.8s ease-in-out infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.25;
    }
  }

  div.panel {
    background: var(--panel);
    border: 1px solid var(--border-dim);
    border-radius: 6px;
    margin: 18px 0 0;
  }

  div.panel .panel-body {
    padding: 18px;
  }
  .panel-section .section-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }
  .panel-section .section-label {
    font-family: var(--mono);
    font-size: 10.5px;
    letter-spacing: 0.22em;
    text-transform: uppercase;
    color: var(--ink-muted);
  }
  .panel-divider {
    height: 1px;
    background: var(--border-dim);
    margin: 16px 0;
  }

  .chips {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }
  .chip {
    font-family: var(--mono);
    font-size: 10.5px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    padding: 6px 11px;
    background: var(--signal-soft);
    border: 1px solid var(--signal);
    border-radius: 3px;
    color: var(--signal);
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    user-select: none;
    transition:
      opacity 0.15s ease,
      background 0.15s ease,
      color 0.15s ease;
  }
  .chip .count {
    color: var(--ink-muted);
  }
  .chip.off {
    background: transparent;
    border-color: var(--border-dim);
    color: var(--ink-dim);
  }
  .chip.off .count {
    color: var(--ink-dim);
  }
  .chip:hover {
    opacity: 0.85;
  }

  .section-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 18px 0 14px;
  }
  .section-label {
    font-family: var(--mono);
    font-size: 11px;
    letter-spacing: 0.22em;
    text-transform: uppercase;
    color: var(--ink-muted);
  }
  .section-meta {
    font-family: var(--mono);
    font-size: 10.5px;
    letter-spacing: 0.16em;
    text-transform: uppercase;
    color: var(--ink-dim);
  }

  .manga-area {
    overflow: auto;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(370px, 1fr));
    grid-auto-rows: max-content;
    gap: 16px;
    align-content: start;
    padding: 0 0 22px;
    flex: 1;
  }

  select.filter {
    background: transparent;
    border: 1px solid var(--border-mid);
    color: var(--ink);
    font-family: var(--mono);
    font-size: 11px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    padding: 9px 12px;
    border-radius: 4px;
    outline: none;
    appearance: none;
    cursor: pointer;
  }
  select.filter:hover {
    border-color: var(--ink-muted);
  }
  select.filter option {
    background: var(--panel);
    color: var(--ink);
  }

  .overall {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 20px 0 18px;
    border-bottom: 1px solid var(--border-dim);
  }

  .overall-head {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
  }

  .overall-label {
    font-family: var(--mono);
    font-size: 11px;
    letter-spacing: 0.22em;
    text-transform: uppercase;
    color: var(--ink-muted);
  }
  .overall-pct {
    font-family: var(--mono);
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 0.06em;
    color: var(--signal);
  }
  .overall-meter {
    display: flex;
    gap: 2px;
    height: 6px;
  }

  .tick {
    flex: 1;
    background: rgba(255, 255, 255, 0.08);
    border-radius: 0;
  }
  .tick.on {
    background: var(--signal);
  }

  .overall-foot {
    display: flex;
    justify-content: space-between;
    font-family: var(--mono);
    font-size: 10.5px;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--ink-dim);
  }

  input[type="color"] {
    width: 46px;
    height: 34px;
    padding: 2px;
    background: transparent;
    border: 1px solid var(--border-mid);
    border-radius: 4px;
    cursor: pointer;
  }
</style>

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

  let disabled = false;
  let totalSize = 0;
  var list: any[] = [];
  let sort = "default";
  let selected = "remaining";
  let mangaFinal: any[];
  $: searchValue = "";
  let ignoreHosts: string[] = [];
  let color = "#3ddc84";

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

      if (selected === "no_error" && x["error"]) {
        return false;
      }

      if (selected !== "no_error" && x["error"]) {
        return false;
      }

      return x["remaining_chapters"] > 0;
    });

    let mangaSortedAndFiltered = mangaFiltered.toSorted((a, b) => {
      if (sort == "index") {
        return a["index"] - b["index"];
      } else if (sort == "remaining") {
        return a["remaining_chapters"] - b["remaining_chapters"];
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
          sort = "default";
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
    // Convert hex -> RGB
    const hex = primaryHex.replace("#", "");
    const r = parseInt(hex.slice(0, 2), 16);
    const g = parseInt(hex.slice(2, 4), 16);
    const b = parseInt(hex.slice(4, 6), 16);

    // RGB -> HSL
    function rgbToHsl(r: number, g: number, b: number) {
      r /= 255;
      g /= 255;
      b /= 255;

      const max = Math.max(r, g, b);
      const min = Math.min(r, g, b);

      let h = (max + min) / 2;
      let s = (max + min) / 2;
      let l = (max + min) / 2;

      if (max === min) {
        h = s = 0;
      } else {
        const d = max - min;

        s = l > 0.5 ? d / (2 - max - min) : d / (max + min);

        switch (max) {
          case r:
            h = (g - b) / d + (g < b ? 6 : 0);
            break;
          case g:
            h = (b - r) / d + 2;
            break;
          case b:
            h = (r - g) / d + 4;
            break;
        }

        h /= 6;
      }

      return [h * 360, s * 100, l * 100];
    }

    // HSL -> HEX
    function hslToHex(h: number, s: number, l: number) {
      s /= 100;
      l /= 100;

      const c = (1 - Math.abs(2 * l - 1)) * s;
      const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
      const m = l - c / 2;

      let r = 0,
        g = 0,
        b = 0;

      if (h < 60) [r, g, b] = [c, x, 0];
      else if (h < 120) [r, g, b] = [x, c, 0];
      else if (h < 180) [r, g, b] = [0, c, x];
      else if (h < 240) [r, g, b] = [0, x, c];
      else if (h < 300) [r, g, b] = [x, 0, c];
      else [r, g, b] = [c, 0, x];

      const toHex = (n: number) =>
        Math.round((n + m) * 255)
          .toString(16)
          .padStart(2, "0");

      return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
    }

    const [h, s] = rgbToHsl(r, g, b);

    const root = document.documentElement;

    root.style.setProperty(`--void`, hslToHex(h, Math.max(8, s * 0.15), 3));
    root.style.setProperty(`--panel`, hslToHex(h, Math.max(10, s * 0.2), 6));
    root.style.setProperty(
      `--panel-soft`,
      hslToHex(h, Math.max(12, s * 0.25), 9),
    );
    root.style.setProperty(`--border-dim`, `rgba(${r},${g},${b},0.14)`);
    root.style.setProperty(`--border-mid`, `rgba(${r},${g},${b},0.32)`);
    root.style.setProperty(`--signal`, primaryHex);
    root.style.setProperty(`--signal-soft`, `rgba(${r},${g},${b},0.12)`);
    root.style.setProperty(`--amber`, "#f5a623");
    root.style.setProperty(`--fault`, "#ff5c5c");
    root.style.setProperty(`--ink`, hslToHex(h, Math.min(40, s * 0.4), 93));
    root.style.setProperty(
      `--ink-muted`,
      hslToHex(h, Math.min(25, s * 0.25), 48),
    );
    root.style.setProperty(
      `--ink-dim`,
      hslToHex(h, Math.min(18, s * 0.18), 23),
    );
  }

  async function changeColor() {
    await dataStore.set("primary-color", color);
    generateTheme(color);
  }
</script>

<div data-tauri-drag-region class="app-bar">
  <Titlebar>
    <div class="top-level-filter">
      <div style="margin: 10px">
        <Cinput placeholder="search threads…" value={searchValue}>
          <circle cx="11" cy="11" r="7" /><line
            x1="21"
            y1="21"
            x2="16.65"
            y2="16.65"
          />
        </Cinput>
      </div>

      <select class="filter" bind:value={selected} style="margin: 10px">
        <option value="remaining">Remaining</option>
        <option value="all">Show All</option>
        <option value="no_error">No Error</option>
      </select>

      <select class="filter" bind:value={sort} style="margin: 10px">
        <option value="default">Default</option>
        <option value="index">Index</option>
        <option value="remaining">Remaining</option>
      </select>

      <button
        class="fetch"
        on:click={eventEmit}
        {disabled}
        style="margin: 10px"
      >
        {#if !disabled}
          <span class="pulse"></span>
        {/if}
        fetch now
      </button>
    </div>
  </Titlebar>
</div>
<div class="app">
  <details class="panel">
    <summary>
      <span class="l">filters &amp; config</span>
      <span class="right"
        ><span class="badge" id="sourcesBadge"></span><span class="chev">▸</span
        ></span
      >
    </summary>
    <div class="panel-body">
      <div class="panel-section">
        <div class="section-head">
          <span class="section-label">by source</span>
          <div class="sources-actions">
            <button class="mini-btn" id="selectAll">select all</button>
            <button class="mini-btn" id="clearAll">clear all</button>
          </div>
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
  </details>
  <div class="overall">
    <div class="overall-head">
      <span class="overall-label"
        >{list.length} : {mangaFinal.length} of {totalSize}</span
      >
      <span class="overall-pct" id="overallPct"
        >{Math.ceil((list.length / totalSize) * 100)}%</span
      >
    </div>
    <div class="overall-meter" id="overallMeter">
      {#each { length: 20 } as _, i}
        <span class="tick {i < (list.length / totalSize) * 20 ? 'on' : ''}"
        ></span>
      {/each}
    </div>
  </div>

  <div class="manga-area">
    {#each mangaFinal as manga}
      <MangaView mangaData={manga} />
    {/each}
  </div>
</div>

<style>
  .app-bar {
    width: 100%;
  }

  .app {
    width: 100%;
    display: flex;
    flex-direction: column;
    height: calc(100% - 55px);
  }

  .top-level-filter {
    display: flex;
  }

  .fetch {
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
  .fetch:hover {
    background: rgba(61, 220, 132, 0.22);
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

  details.panel {
    background: var(--panel);
    border: 1px solid var(--border-dim);
    border-radius: 8px;
    margin-bottom: 14px;
  }
  details.panel summary {
    cursor: pointer;
    list-style: none;
    padding: 12px 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  details.panel summary::-webkit-details-marker {
    display: none;
  }
  details.panel summary .l {
    font-family: var(--mono);
    font-size: 10px;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--ink-dim);
  }
  details.panel summary .right {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  details.panel summary .badge {
    font-family: var(--mono);
    font-size: 11px;
    color: var(--ink-muted);
  }
  details.panel summary .chev {
    display: inline-block;
    font-family: var(--mono);
    font-size: 11px;
    color: var(--ink-dim);
    transition: transform 0.15s ease;
  }
  details.panel[open] summary .chev {
    transform: rotate(90deg);
  }
  details.panel .panel-body {
    padding: 0 16px 16px;
  }
  .panel-section .section-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }
  .panel-section .section-label {
    font-family: var(--mono);
    font-size: 10px;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--ink-dim);
  }
  .panel-divider {
    height: 1px;
    background: var(--border-dim);
    margin: 14px 0;
  }
  .mini-btn {
    font-family: var(--mono);
    font-size: 11px;
    color: var(--ink-muted);
    background: none;
    border: 1px solid var(--border-dim);
    padding: 5px 11px;
    border-radius: 6px;
    cursor: pointer;
  }
  .mini-btn:hover {
    color: var(--ink);
    border-color: var(--border-mid);
  }
  .chips {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }
  .chip {
    font-family: var(--mono);
    font-size: 11.5px;
    padding: 6px 18px 6px 11px;
    background: var(--signal-soft);
    border: 1px solid var(--signal);
    color: var(--signal);
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    gap: 7px;
    user-select: none;
    transition:
      opacity 0.15s ease,
      background 0.15s ease;
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

  .manga-area {
    overflow: auto;
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    align-content: baseline;
    padding: 10px;
    height: 100%;
  }

  select.filter {
    background: var(--panel);
    border: 1px solid var(--border-dim);
    color: var(--ink);
    font-family: var(--mono);
    font-size: 12px;
    padding: 9px 10px;
    border-radius: 6px;
    outline: none;
    appearance: none;
  }

  .overall {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .overall-head {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
  }

  .overall-label {
    font-family: var(--mono);
    font-size: 10px;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--ink);
  }
  .overall-pct {
    font-family: var(--mono);
    font-size: 13px;
    font-weight: 600;
    color: var(--signal);
  }
  .overall-meter {
    display: flex;
    gap: 2px;
    height: 8px;
  }

  .tick {
    flex: 1;
    background: var(--ink-dim);
    border-radius: 1px;
  }
  .tick.on {
    background: var(--signal);
  }
</style>

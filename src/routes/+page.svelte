<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import Titlebar from "../lib/titlebar.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import dataStore from "$lib/data_store";
  import Cinput from "$lib/cinput.svelte";
  import ConfigFilesView from "$lib/config-files-view.svelte";
  import MangaView from "$lib/manga-view.svelte";

  let searchValue: string = "";
  let disabled = false;
  let totalSize = 0;
  var list: any[] = [];
  let sort = "Default";

  onMount(async () => {
    await listen("scan_data", (event) => {
      console.log(event.payload);
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
  });

  function eventEmit() {
    if (!disabled) {
      invoke("get_manga");
    }
  }
</script>

<div data-tauri-drag-region class="app-bar">
  <Titlebar>
    <div class="top-level-filter">
      <Cinput placeholder="search threads…" value={searchValue}>
        <circle cx="11" cy="11" r="7" /><line
          x1="21"
          y1="21"
          x2="16.65"
          y2="16.65"
        />
      </Cinput>

      <button class="fetch" on:click={eventEmit}><span class="pulse"></span>fetch now</button>
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
          <span class="chip" data-source=""
            >Asura <span class="count">(1)</span></span
          >
          <span class="chip off" data-source=""
            >Asura <span class="count">(1)</span></span
          >
        </div>
      </div>

      <div class="panel-divider"></div>

      <ConfigFilesView />
    </div>
  </details>
  <div class="rail" id="rail">
    <span style="flex:1; background:var(--signal)"></span><span
      style="flex:5; background:var(--amber)"
    ></span><span style="flex:3; background:var(--ink-dim)"></span><span
      style="flex:3; background:var(--fault)"
    ></span>
  </div>

  <div class="manga-area">
    {#each list as manga}
      <MangaView />
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
    height: calc(100% - 45px);
  }

  .top-level-filter {
    padding-top: 10px;
    padding-left: 10px;
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
    clip-path: polygon(
      0 0,
      calc(100% - 9px) 0,
      100% 50%,
      calc(100% - 9px) 100%,
      0 100%
    );
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

  .rail {
    display: flex;
    gap: 2px;
    height: 2px;
  }
  .rail span {
    border-radius: 1px;
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
</style>

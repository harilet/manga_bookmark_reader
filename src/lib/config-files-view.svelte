<script lang="ts">
  import { onMount } from "svelte";
  import dataStore from "$lib/data_store";
  import { invoke } from "@tauri-apps/api/core";

  $: rules_path = "";
  $: site_parser_rules_path = "";

  onMount(async () => {
    let temp_rules_path = await dataStore.get("set_rules_path");
    let temp_site_parser_rules_path = await dataStore.get(
      "set_site_parser_rules_path",
    );
    if (typeof temp_rules_path === "string") {
      rules_path = temp_rules_path;
      invoke("set_rules_path", {
        path: temp_rules_path,
      });
    }

    if (typeof temp_site_parser_rules_path === "string") {
      site_parser_rules_path = temp_site_parser_rules_path;
      invoke("set_site_parser_rules_path", {
        path: temp_site_parser_rules_path,
      });
    }
  });
</script>

<div class="panel-section">
  <div class="section-head">
    <span class="section-label">config files</span>
  </div>
  <div class="config-fields">
    <label>
      <span class="field-label">rules config path</span>
      <input
        type="text"
        id="cfgThreads"
        placeholder="/etc/scraper/threads.json"
        bind:value={rules_path}
      />
    </label>
    <label>
      <span class="field-label">site parser config path</span>
      <input
        type="text"
        id="cfgOutput"
        placeholder="/var/data/scraper/output.json"
        bind:value={site_parser_rules_path}
      />
    </label>
  </div>
  <div class="config-save-row">
    <button class="mini-btn" id="saveConfig">save</button>
  </div>
</div>

<style>
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

  .config-fields {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 14px;
  }
  .config-fields label {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .field-label {
    font-family: var(--mono);
    font-size: 11px;
    color: var(--ink-muted);
  }
  .config-fields input {
    background: var(--panel-soft);
    border: 1px solid var(--border-dim);
    color: var(--ink);
    font-family: var(--mono);
    font-size: 12px;
    padding: 9px 11px;
    border-radius: 6px;
    outline: none;
  }
  .config-fields input::placeholder {
    color: var(--ink-dim);
  }
  .config-fields input:focus {
    border-color: var(--border-mid);
  }
</style>

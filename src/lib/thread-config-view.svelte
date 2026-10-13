<script lang="ts">
  import { onMount } from "svelte";
  import Cinput from "./cinput.svelte";
  import dataStore from "./data_store";
  import { invoke } from "@tauri-apps/api/core";

  $: threadCount = 0;

  onMount(async () => {
    let tempThreadCount = await dataStore.get("thread_count");
    if (typeof tempThreadCount === "string") {
      threadCount = Number.parseInt(tempThreadCount);
      changeThreadCount();
    } else if (tempThreadCount === null) {
      threadCount = 1;
      changeThreadCount();
    }
  });

  function changeThreadCount() {
    invoke("update_thread_count", {
      newThreadCount: parseInt(threadCount + ""),
    });
  }

  async function onThreadCountChange(data: any) {
    let dataValue = data.target.value;
    if (dataValue !== "" && dataValue != "0") {
      dataValue = parseInt(dataValue);
      await dataStore.set("thread_count", dataValue + "");
      threadCount = dataValue;
      changeThreadCount();
    }
  }
</script>

<div class="panel-section">
  <div class="section-head">
    <span class="section-label">thread</span>
  </div>
  <div class="config-fields">
    <Cinput value={threadCount} onChange={onThreadCountChange} />
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
    font-size: 10.5px;
    letter-spacing: 0.22em;
    text-transform: uppercase;
    color: var(--ink-muted);
  }

  .config-fields {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 14px;
  }
</style>

<script lang="ts">
  import { onMount } from "svelte";
  import Input from "./input.svelte";

  export let value = 0;
  export let max = 0;
  export let threadValue = 0;
  export let maxThread = 0;
  export let listCount = 0;
  export let threadCount = "";
  export let onThreadCountChange = (data: any) => {};

  let progress_bar_class = "";
  let mangaInThreadDialog: HTMLDialogElement;

  $: {
    if (value >= max) {
      progress_bar_class = "progress_done";
    } else {
      progress_bar_class = "in_progress";
    }
  }

  onMount(() => {
    mangaInThreadDialog.addEventListener("click", function (event) {
      var rect = mangaInThreadDialog.getBoundingClientRect();
      var isInDialog =
        rect.top <= event.clientY &&
        event.clientY <= rect.top + rect.height &&
        rect.left <= event.clientX &&
        event.clientX <= rect.left + rect.width;
      if (!isInDialog) {
        mangaInThreadDialog.close();
      }
    });
  });

  function showThreadCountInput() {
    if (mangaInThreadDialog.open) {
      mangaInThreadDialog.close();
    } else {
      mangaInThreadDialog.showModal();
    }
  }

  function onThreadCountChange2(data: any) {
    mangaInThreadDialog.close();
    onThreadCountChange(data);
  }
</script>

<dialog bind:this={mangaInThreadDialog} on:close>
  <Input
    bind:value={threadCount}
    placeholder="Number of threads"
    onChange={onThreadCountChange2}
  />
</dialog>

<div class="progress-area-wrapper">
  <div
    data-augmented-ui="bl-clip t-clip-x br-clip border"
    class="progress-area"
  >
    <progress
      data-augmented-ui="bl-clip t-clip-x br-clip"
      class="progress {progress_bar_class}"
      {value}
      {max}
    >
    </progress>
  </div>
</div>

<div class="progress-test">
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div style="display: flex;cursor: pointer;" on:click={showThreadCountInput}>
    {#if !Number.isNaN(maxThread)}
      {#each [...Array(maxThread).keys()] as i (i)}
        <div
          class="thread-box"
          style="--fill: {threadValue <= i
            ? 'transparent'
            : 'var(--primary-color)'};"
        >
          <span></span>
        </div>
      {/each}
    {/if}
  </div>

  <div>
    {listCount} : {value} of {max}
  </div>
</div>

<style>
  .thread-box {
    flex: 1;
    transition: opacity 0.3s ease;
    background-color: var(--fill);
    border: var(--border);
    width: 10px;
    height: 10px;
    margin: 0px 1px;
  }

  .progress-area-wrapper {
    padding: 0px;
  }

  .progress-area {
    display: flex;
    padding: 4px 2px;
    justify-content: space-evenly;
    --aug-border-all: 1px;
    --aug-border-bg: var(--primary-color);
  }

  .progress-test {
    padding: 5px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    color: var(--primary-color);
  }

  .progress {
    width: 100%;
    border-radius: 0px;
    height: 20px;
  }

  .in_progress::-webkit-progress-value {
    background: repeating-linear-gradient(
      90deg,
      var(--in-progress-color),
      var(--in-progress-color) 1px,
      var(--background-color) 1px,
      var(--background-color) 3px
    );
  }
  .progress_done::-webkit-progress-value {
    background: repeating-linear-gradient(
      90deg,
      var(--done-color),
      var(--done-color) 1px,
      var(--background-color) 1px,
      var(--background-color) 3px
    );
  }
  .progress[value]::-webkit-progress-bar {
    background-color: transparent;
  }

  .thread-progress {
    background: transparent;
    border: var(--border);
  }

  .thread-progress[value]::-webkit-progress-bar {
    background-color: transparent;
  }

  .thread-progress::-webkit-progress-value {
    background: var(--done-color);
  }

  dialog {
    border-radius: 5px;
    border-width: 1px;
    transition: all 2s;
    z-index: 2;
    padding: 20px;
    background: var(--background-color);
    border: var(--border);
  }

  dialog::backdrop {
    background: linear-gradient(rgba(0, 0, 0, 0.4), rgba(0, 0, 0, 0.7));
    animation: fade-in 1s;
  }
</style>

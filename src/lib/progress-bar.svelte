<script lang="ts">
  export let value = 0;
  export let max = 0;
  export let threadValue = 0;
  export let maxThread = 0;
  export let listCount = 0;

  let progress_bar_class = "";

  $: {
    if (value >= max) {
      progress_bar_class = "progress_done";
    } else {
      progress_bar_class = "in_progress";
    }
  }
</script>

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
  <div style="display: flex;">
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
</style>

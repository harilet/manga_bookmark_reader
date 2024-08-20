<script lang="ts">
  export let value = 0;
  export let max = 0;

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
  <div data-augmented-ui="tl-clip t-clip-x br-clip" class="progress-area">
    <progress
      data-augmented-ui="tl-clip t-clip-x br-clip"
      class="progress {progress_bar_class}"
      {value}
      {max}
    >
    </progress>
  </div>
</div>

<div class="progress-test">
  <div>
    {value} of {max}
  </div>
</div>

<style>
  .progress-area-wrapper {
    background: var(--primary-color);
    padding: 1px;
  }

  .progress-area {
    display: flex;
    background: var(--background-color);
    padding: 4px 2px;
    justify-content: space-evenly;
  }

  .progress-test {
    padding-top: 5px;
    padding-bottom: 5px;
    display: flex;
    align-items: center;
    justify-content: space-around;
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
</style>

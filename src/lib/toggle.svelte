<script lang="ts">
  export let toggled = false;
  export let label='';

  function toggleState() {
    toggled = !toggled;
  }
</script>

<div class="toggle">
  <div class="label">{label}</div>

  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="toggle-button-wrapper" on:click="{toggleState}">
    <div class="toggle-button" data-augmented-ui="tl-clip br-clip">
      <div
        class="toggle-button-fill {toggled
          ? 'toggle-button-fill-enabled'
          : 'toggle-button-fill-disabled'}"
        data-augmented-ui="tl-clip br-clip"
      ></div>
    </div>
  </div>
</div>

<style>
  .toggle {
    display: flex;
    align-items: center;
  }

  .label {
    color: var(--primary-color);
    margin-right: 10px;
  }

  .toggle-button-wrapper {
    background: var(--primary-color);
    width: 20px;
    padding: 1px;
  }

  .toggle-button {
    height: 20px;
    padding: 1px;
    background: var(--background-color);
  }

  .toggle-button-fill {
    height: 100%;
  }

  .toggle-button-fill-enabled {
    background: var(--primary-color);
  }

  .toggle-button-fill-disabled {
    background: var(--background-color);
  }
</style>

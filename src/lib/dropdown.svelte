<script lang="ts">
  export let sort = "Default";
  let showDropDown = false;

  function dropdownBtnClicked() {
    showDropDown = !showDropDown;
  }

  function setOption(option: string) {
    sort = option;
    showDropDown = false;
  }
</script>

<div class="dropdown">
  <button on:click="{dropdownBtnClicked}" class="dropbtn">
    {sort}
  </button>
  {#if showDropDown}
    <div class="dropdown-content">
      <button on:click="{() => setOption('Default')}">Default</button>
      <button on:click="{() => setOption('Index')}">Index</button>
      <button on:click="{() => setOption('Remaining')}">Remaining</button>
    </div>
  {/if}
</div>

<style>
  .dropdown {
    width: 100%;
  }

  .dropbtn {
    width: 100%;
    background: var(--background-color);
    color: var(--primary-color);
    border: solid 1px var(--primary-color);
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .dropdown-content {
    width: calc(100% - 6px);
    position: absolute;
    background-color: var(--background-color);
    color: var(--primary-color);

    min-width: 160px;
    box-shadow: 0px 8px 16px 0px rgba(0, 0, 0, 0.2);
    z-index: 1;
    display: flex;
    flex-direction: column;
  }

  .dropdown-content button {
    border: solid 2px var(--primary-color);
    border-top: none;
    background: var(--background-color);
    color: var(--primary-color);
  }
</style>

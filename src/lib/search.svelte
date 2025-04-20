<script lang="ts">
  import CloseIcon from "./icon/closeIcon.svelte";
  import SearchIcon from "./icon/searchIcon.svelte";
  import Input from "./input.svelte";

  export let searchText = "";

  $: showSearchBox = false;

  function handleClick() {
    showSearchBox = !showSearchBox;
    searchText = "";
  }

  function inputOnKeydown(data: any) {
    if (data.key == "Escape") {
      handleClick();
    }
  }
</script>

<div class="search-box">
  <button
    class="search-btn {showSearchBox ? 'hide' : 'show'}"
    on:click={handleClick}
  >
    <SearchIcon />
  </button>
  <div class="search-main {showSearchBox ? 'show' : 'hide'}">
    <Input
      bind:value={searchText}
      placeholder="Search..."
      keydown={inputOnKeydown}
    />
    <button class="search-btn" on:click={handleClick}>
      <CloseIcon />
    </button>
  </div>
</div>

<style>
  .search-box {
    background: transparent;
    border: solid 1px var(--primary-color);
    height: 30px;
    margin: 0px;
    padding: 0px;
    margin-left: 20px;
  }

  .search-main {
    display: flex;
    align-items: center;
    justify-items: center;
  }

  .search-btn {
    background: transparent;
    border: none;
    cursor: pointer;
  }

  .show {
    display: flex !important;
  }

  .hide {
    display: none !important;
  }
</style>

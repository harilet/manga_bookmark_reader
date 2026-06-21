<script lang="ts">
  export let placeholder: string = "";
  export let value: string = "";
</script>

<div class="search">
  <svg
    width="13"
    height="13"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
  >
    <slot />
  </svg>

  <input type="text" id="search" {placeholder} bind:value={value} />
</div>

<style>
  .search {
    background: var(--panel);
    border: 1px solid var(--border-dim);
    border-radius: 6px;
    padding: 8px 12px;
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 200px;
  }

  .search svg {
    color: var(--ink-dim);
    flex: none;
  }

  .search input {
    background: none;
    border: none;
    outline: none;
    color: var(--ink);
    font-family: var(--mono);
    font-size: 12px;
  }

  .search input::placeholder {
    color: var(--ink-dim);
  }
</style>

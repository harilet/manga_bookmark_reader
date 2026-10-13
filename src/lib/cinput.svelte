<script lang="ts">
  import type { ChangeEventHandler } from "svelte/elements";

  export let placeholder: string = "";
  export let value: any = "";
  export let onChange: ChangeEventHandler<HTMLInputElement> | undefined | null =
    null;
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

  <input
    type="text"
    id="search"
    {placeholder}
    bind:value
    on:change={onChange}
  />
</div>

<style>
  .search {
    background: transparent;
    border: 1px solid var(--border-mid);
    border-radius: 4px;
    padding: 9px 12px;
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 220px;
    transition: border-color 0.15s ease;
  }

  .search:focus-within {
    border-color: var(--signal);
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
    font-size: 11px;
    letter-spacing: 0.06em;
    width: 100%;
  }

  .search input::placeholder {
    color: var(--ink-dim);
    text-transform: uppercase;
    letter-spacing: 0.12em;
  }
</style>

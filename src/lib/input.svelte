<script lang="ts">
  export let value = "";
  export let placeholder = "";
  export let keydown;
</script>

<input
  type="text"
  bind:value
  {placeholder}
  on:keydown={keydown}
/>

<style>
  input[type="text"] {
    height: 20px;
    background: transparent;
    color: var(--primary-color);
    border: none;
  }

  input[type="text"]:focus-visible {
    outline: none !important;
  }
</style>

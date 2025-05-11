<script lang="ts">
  export let value = "";
  export let placeholder = "";
  export let keydown = (data: any) => {};
  export let disabled = false;
  export let onChange = (data: any) => {};
</script>

<input
  type="text"
  bind:value
  {placeholder}
  {disabled}
  on:keydown={keydown}
  on:change={onChange}
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

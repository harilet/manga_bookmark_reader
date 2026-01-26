<script lang="ts">
    import Input from "./input.svelte";

    export let searchText = "";
</script>

<div class="search-box all-corner-stroke">
    <Input bind:value={searchText} placeholder="Search..." />
</div>

<style>
    .search-box {
        border: solid 1px var(--primary-color);
        height: 30px;
        margin: 0px;
        padding: 1px 5px;
        margin-left: 20px;
        display: flex;
        align-items: center;
        justify-items: center;
    }
</style>

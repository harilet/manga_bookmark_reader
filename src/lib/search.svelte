<script lang="ts">
    import Input from "./input.svelte";

    export let searchText = "";
</script>

<div class="search-box" data-augmented-ui="tr-clip bl-clip border">
    <Input bind:value={searchText} placeholder="Search..." />
</div>

<style>
    .search-box {
        background: transparent;
        border: solid 1px var(--primary-color);
        height: 30px;
        margin: 0px;
        padding: 1px 5px;
        margin-left: 20px;
        display: flex;
        align-items: center;
        justify-items: center;

        --aug-border-all: 1px;
        --aug-border-bg: var(--accent);

        --w: 3px; /* stroke width */
        --l: 26px; /* corner length */

        background-image:
          /* top-left horizontal */
            linear-gradient(var(--primary-color) 0 0),
            /* top-left vertical */ linear-gradient(var(--primary-color) 0 0),
            /* bottom-right horizontal */
            linear-gradient(var(--primary-color) 0 0),
            /* bottom-right vertical */
            linear-gradient(var(--primary-color) 0 0),
            /* panel background */
            linear-gradient(
                    180deg,
                    var(--background-color),
                    var(--background-color)
                );

        background-size:
            var(--l) var(--w),
            var(--w) var(--l),
            var(--l) var(--w),
            var(--w) var(--l),
            100% 100%;

        background-position:
            left top,
            left top,
            right bottom,
            right bottom,
            0 0;

        background-repeat: no-repeat;
    }
</style>

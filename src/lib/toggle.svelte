<script lang="ts">
    export let toggled = false;

    function toggleState() {
        toggled = !toggled;
    }
</script>

{#if toggled}
    <button
        data-augmented-ui="tr-clip bl-clip border"
        class="toggle toggle-button-fill-enabled"
        on:click={toggleState}
    >
        Showing: All Manga
    </button>
{:else}
    <button
        data-augmented-ui="tr-clip bl-clip border"
        class="toggle toggle-button-fill-disabled"
        on:click={toggleState}
    >
        Show All Manga
    </button>
{/if}

<style>
    .toggle {
        width: 100%;
        background: var(--background-color);
        color: var(--primary-color);
        border: none;
        height: 30px;
        display: flex;
        align-items: center;
        justify-content: center;
        margin-bottom: 5px;
        --aug-border-all: 1px;
        --aug-border-bg: var(--accent);
    }

    .toggle-button-fill-enabled {
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
            linear-gradient(180deg, var(--accent), var(--accent));

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

    .toggle-button-fill-disabled {
        background: var(--background-color);
    }
</style>

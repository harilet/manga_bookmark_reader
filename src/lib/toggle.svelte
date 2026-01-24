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
        background: var(--accent);
    }

    .toggle-button-fill-disabled {
        background: var(--background-color);
    }
</style>

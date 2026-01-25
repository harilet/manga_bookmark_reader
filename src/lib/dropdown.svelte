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

<button
    data-augmented-ui="tr-clip bl-clip border"
    on:click={dropdownBtnClicked}
    class="dropbtn"
>
    {sort}
</button>
{#if showDropDown}
    <div class="dropdown-content" data-augmented-ui="tr-clip bl-clip border">
        <button on:click={() => setOption("Default")}>Default</button>
        <button on:click={() => setOption("Index")}>Index</button>
        <button on:click={() => setOption("Remaining")}>Remaining</button>
    </div>
{/if}

<style>
    .dropbtn {
        width: 100%;
        color: var(--primary-color);
        border: none;
        height: 30px;
        display: flex;
        align-items: center;
        justify-content: center;
        margin-bottom: 5px;
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

    .dropdown-content {
        width: auto;
        position: absolute;
        background-color: var(--background-color);
        color: var(--primary-color);
        padding: 10px;
        min-width: 160px;
        z-index: 1;
        display: flex;
        flex-direction: column;
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

    .dropdown-content button {
        border: var(--border);
        background: var(--background-color);
        color: var(--primary-color);
        margin: 5px;
    }
</style>

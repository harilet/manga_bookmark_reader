<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import Titlebar from "../lib/titlebar.svelte";
    import Toggle from "../lib/toggle.svelte";
    import ProgressBar from "../lib/progress-bar.svelte";
    import Dropdown from "../lib/dropdown.svelte";
    import Manga from "../lib/manga.svelte";
    import Search from "$lib/search.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import Random from "../lib/icon/random.svelte";
    import dataStore from "$lib/data_store";
    import Refresh from "$lib/icon/refresh.svelte";
    import DeleteHistory from "$lib/icon/delete_history.svelte";
    import Lists from "$lib/icon/lists.svelte";
    import GridOn from "$lib/icon/grid_on.svelte";

    let showAllMangas = false;
    let totalSize = 0;
    var list: any[] = [];
    let disabled = false;
    let sort = "Default";
    let mangaFiltered: any[];
    let mangaSortedAndFiltered: any[];
    let mangaFinal: any[];
    let mangaSample: any[] = [];
    let shuffleListDialog: HTMLDialogElement;
    let filterDialog: HTMLDialogElement;
    let mangaListMode = "List";
    let showDraw = false;
    let ignoreHosts: string[] = [];
    let showErrors = true;
    let insideDrawer = false;
    let color = "#81a1c1";

    $: shuffleSample = false;
    $: sampleCount = 5;
    $: listLength = list.length;
    $: finalLength = mangaFinal.length;
    $: searchText = "";
    $: savedUrlListLength = 0;

    $: {
      const root = document.documentElement;
      root.style.setProperty(`--primary-color`, color);

        mangaFiltered = list.filter((x) => {
            for (let host of ignoreHosts) {
                if (x["url"].includes(host)) {
                    return false;
                }
            }

            if (showErrors && x["error"]) {
                return true;
            }

            if (!showErrors && x["error"]) {
                return false;
            }

            if (showAllMangas) {
                return true;
            }

            return x["remaining_chapters"] > 0;
        });

        mangaSortedAndFiltered = mangaFiltered.toSorted((a, b) => {
            if (sort == "Index") {
                return a["index"] - b["index"];
            } else if (sort == "Remaining") {
                return a["remaining_chapters"] - b["remaining_chapters"];
            }
            return 0;
        });

        mangaFinal = mangaSortedAndFiltered.filter((x) => {
            let url: string = x["url"];
            let name: string = x["name"];
            if (url.search(searchText) != -1) {
                return true;
            } else if (name.search(searchText) != -1) {
                return true;
            }
            return false;
        });
    }

    onMount(async () => {
        dataStore.get("savedUrlList").then((data: any) => {
            if (data != null) {
                savedUrlListLength = data.length;
            }
        });

        await listen("scan_data", (event) => {
            if (event.payload == "done") {
                disabled = false;
            } else {
                const data = JSON.parse(JSON.stringify(event.payload));
                if (data["size"]) {
                    disabled = true;
                    totalSize = data["size"];
                    list = [];
                    sort = "Default";
                } else if (data["name"]) {
                    list = [...list, data];
                }
            }
        });

        shuffleListDialog.addEventListener("click", function (event) {
            var rect = shuffleListDialog.getBoundingClientRect();
            var isInDialog =
                rect.top <= event.clientY &&
                event.clientY <= rect.top + rect.height &&
                rect.left <= event.clientX &&
                event.clientX <= rect.left + rect.width;
            if (!isInDialog) {
                shuffleListDialog.close();
            }
        });

        filterDialog.addEventListener("click", function (event) {
            var rect = filterDialog.getBoundingClientRect();
            var isInDialog =
                rect.top <= event.clientY &&
                event.clientY <= rect.top + rect.height &&
                rect.left <= event.clientX &&
                event.clientX <= rect.left + rect.width;
            if (!isInDialog) {
                filterDialog.close();
            }
        });
    });

    function eventEmit() {
        if (!disabled) {
            invoke("get_manga");
        }
    }

    function openRandom() {
        shuffleListDialog.showModal();
    }

    async function dataSaving() {
        mangaSample = [];
        for (let i = 0; i < sampleCount; i++) {
            let data: any[] | null = await dataStore.get("savedUrlList");

            if (data != null) {
                let shuffledArray = list.toSorted((a, b) => {
                    return a["index"] - b["index"];
                });

                let savedUrlList: any[] = [...data];
                if (shuffleSample) {
                    shuffledArray = shuffleArray(shuffledArray);
                }

                if (savedUrlList.length == list.length) {
                    await dataStore.set("savedUrlList", []);
                    savedUrlList = [];
                }

                // Loop through each URL in the list
                for (
                    let urlIndex = 0;
                    urlIndex < shuffledArray.length;
                    urlIndex++
                ) {
                    let url = shuffledArray[urlIndex]["url"];

                    // Check if the URL is already in the saved list
                    if (!savedUrlList.includes(url)) {
                        mangaSample = [...mangaSample, shuffledArray[urlIndex]];
                        savedUrlList.push(url); // Add the URL to the saved list
                        await dataStore.set("savedUrlList", savedUrlList);
                        savedUrlListLength = savedUrlList.length;
                        break;
                    }
                }
            } else {
                await dataStore.set("savedUrlList", []);
            }
        }
    }

    function shuffleArray(array: any[]) {
        let returnArray = [...array];
        for (let i = returnArray.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            [returnArray[i], returnArray[j]] = [returnArray[j], returnArray[i]];
        }
        return returnArray;
    }

    function clearRandomHistory() {
        dataStore.delete("savedUrlList");
    }

    function mangaListModeToggle() {
        mangaListMode = mangaListMode === "List" ? "Grid" : "List";
    }

    function toggleDrawer() {
        showDraw = !showDraw;
    }

    function openFilter() {
        filterDialog.showModal();
    }

    function getUniqueHosts(): string[] {
        const hosts = new Set<string>();
        list.forEach((manga) => {
            try {
                const url = new URL(manga.url);
                hosts.add(url.hostname);
            } catch (e) {
                console.error("Invalid URL:", manga.url);
            }
        });
        return Array.from(hosts);
    }

    function toggleIgnoreHost(host: string) {
        ignoreHosts = ignoreHosts.includes(host)
            ? ignoreHosts.filter((h) => h !== host)
            : [...ignoreHosts, host];
    }

    function unselectAll() {
        selectAll();
        getUniqueHosts().forEach(function (manga: string) {
            ignoreHosts = [...ignoreHosts, manga];
        });
    }

    function selectAll() {
        ignoreHosts = [];
    }

    function handleOutsideClick(event: MouseEvent) {
        console.log(showDraw);
        if (!insideDrawer) {
            showDraw = false;
        }
    }
</script>

<dialog
    bind:this={shuffleListDialog}
    on:close
    class="main-scrollbar"
    data-augmented-ui="tr-clip bl-clip border"
>
    <div style="display: flex;flex-direction: column;height: 100%;">
        <div
            style="display: flex;flex-direction: row; justify-content: space-between;"
        >
            <Toggle bind:toggled={shuffleSample} />
            <input
                type="text"
                placeholder="Number of threads"
                bind:value={sampleCount}
                style="text-align: center;"
            />
            <button class="btn" on:click={() => dataSaving()}>
                <Refresh />
            </button>
            <button class="btn" on:click={clearRandomHistory}>
                <DeleteHistory />
            </button>
        </div>
        <div class=" manga-list">
            {#key mangaSample}
                {#each mangaSample as manga_item}
                    <Manga propValue={manga_item} {mangaListMode}></Manga>
                {/each}
            {/key}
        </div>
    </div>
</dialog>

<dialog
    bind:this={filterDialog}
    on:close
    class="main-scrollbar"
    data-augmented-ui="tr-clip bl-clip border"
>
    <div
        style="display: flex;flex-direction: row;height: 100%;color: var(--primary-color);align-items: center;"
    >
        <div
            style="display: flex; flex-direction: column;justify-content: center;width: 160px;"
        >
            <div>
                <button
                    data-augmented-ui="tr-clip bl-clip border"
                    class="btn-mode-toggle"
                    style="width: 100%;"
                    on:click={selectAll}
                >
                    Select All
                </button>
                <button
                    data-augmented-ui="tr-clip bl-clip border"
                    class="btn-mode-toggle"
                    style="width: 100%;"
                    on:click={unselectAll}
                >
                    Unselect All
                </button>
            </div>
        </div>
        <div
            style="overflow: auto;padding: 0px 15px;height: 100%;width: 100%;display: flex;flex-direction: column;flex-wrap: wrap;"
        >
            {#key mangaFinal}
                {#each getUniqueHosts() as host}
                    <div
                        style="display: flex; flex-direction: row;align-items: center;margin-bottom: 10px;width: 150px"
                    >
                        <button
                            data-augmented-ui="tr-clip bl-clip border"
                            style="background-color: var({ignoreHosts.includes(
                                host,
                            )
                                ? '--background-color'
                                : '--accent'});"
                            class="toggle-btn"
                            on:click={() => toggleIgnoreHost(host)}
                        >
                            {host}
                        </button>
                    </div>
                {/each}
            {/key}
        </div>
    </div>
</dialog>

<svelte:window on:click={handleOutsideClick} />

<div
    style={showDraw ? "left: 0" : ""}
    class="drawer"
    data-augmented-ui="tr-clip br-clip border"
    role="navigation"
    on:mouseleave={() => (insideDrawer = false)}
    on:mouseenter={() => (insideDrawer = true)}
>
    <div class="filter-items" style="width: -webkit-fill-available;">
        <Dropdown bind:sort />
    </div>
    <div class="filter-items" style="width: -webkit-fill-available;">
        <Toggle bind:toggled={showAllMangas} text="Show All Manga" />
    </div>

    <div class="filter-items">
        <Search bind:searchText />
    </div>

    <div
        style="display: flex;
    align-items: center;"
        class="filter-items"
    >
        <button class="btn" on:click={openRandom}>
            <Random />
        </button>
        <div class="label">
            {savedUrlListLength}
        </div>
    </div>
    <div class="filter-items">
        <button
            data-augmented-ui="tr-clip bl-clip border"
            class="btn-mode-toggle"
            style="width: -webkit-fill-available;"
            on:click={mangaListModeToggle}
        >
            {#if mangaListMode == "List"}
                <Lists />
            {:else}
                <GridOn />
            {/if}
        </button>
    </div>
    <div class="filter-items">
        <button
            data-augmented-ui="tr-clip bl-clip border"
            class="btn-mode-toggle"
            style="width: -webkit-fill-available;"
            on:click={openFilter}
        >
            Filter
        </button>
    </div>
    <div class="filter-items">
        <Toggle bind:toggled={showErrors} text="Show Error" />
    </div>

    <div class="filter-items">
        <input type="color" id="favcolor" name="favcolor" bind:value={color} />
    </div>
</div>

<div data-tauri-drag-region class="app-bar">
    <Titlebar />
</div>

<div class="app">
    <div class="top-progress-bar-area">
        <ProgressBar
            value={listLength}
            max={totalSize}
            listCount={finalLength}
        />
    </div>

    <div class="main-area">
        <div class="side-bar">
            <button
                data-augmented-ui="tr-clip bl-clip border"
                class="btn-mode-toggle"
                style="width: 200px;
                height: 60px;"
                on:click={toggleDrawer}
                on:mouseleave={() => (insideDrawer = false)}
                on:mouseenter={() => (insideDrawer = true)}
            >
                Filter
            </button>

            <button
                data-augmented-ui="tr-clip bl-clip border"
                class="fetch-button font-style {disabled
                    ? 'fetch-button-disabled'
                    : 'fetch-button-enabled'}"
                on:click={eventEmit}
            >
                {disabled ? "Fetching" : "Fetch"}
            </button>
        </div>
        <div
            class="main-scrollbar {mangaListMode == 'List'
                ? 'manga-list'
                : 'manga-grid'}"
        >
            {#key showAllMangas}
                {#key sort}
                    {#key ignoreHosts}
                        {#key showErrors}
                            {#each mangaFinal as manga}
                                <Manga propValue={manga} {mangaListMode}
                                ></Manga>
                            {/each}
                        {/key}
                    {/key}
                {/key}
            {/key}
        </div>
    </div>
</div>

<style>
    .filter-items {
        height: 100%;
        border-top: var(--border);
        padding: 10px;
        width: -webkit-fill-available;
        display: flex;
        align-items: center;
    }
    .toggle-btn {
        padding: 15px;
        border: solid 1px var(--accent);
        height: 40px;
        width: 100%;
        color: var(--primary-color);
        --aug-border-all: 1px;
        --aug-border-bg: var(--accent);
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .drawer {
        display: flex;
        flex-direction: column;
        position: absolute;
        top: 0;
        left: -30%;
        height: 100%;
        width: 20%;
        z-index: 5;
        background-color: var(--background-color);
        --aug-border-all: 1px;
        --aug-border-bg: var(--accent);
        justify-content: space-between;

        transition: all 0.3s ease-in-out;
    }

    .btn-mode-toggle {
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--primary-color);
        border: var(--border);
        padding: 5px 15px;
        --aug-border-all: 1px;
        --aug-border-bg: var(--accent);
        border: none;

        --w: 2px; /* stroke width */
        --l: 24px; /* corner length */

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

    .top-progress-bar-area {
        background-color: color-mix(
            in oklab,
            var(--color-slate-900) 30%,
            transparent
        );
    }
    .app-bar {
        width: 100%;
        height: 35px;
    }
    .app {
        width: 100%;
        display: flex;
        flex-direction: column;
    }

    .main-area {
        display: flex;
        flex-direction: column;
        height: calc(100vh - 90px);
    }

    .side-bar {
        height: auto;
        width: 100%;
        display: flex;
        align-items: center;
        flex-direction: row;
        position: relative;
        border-bottom: solid 1px var(--accent);
    }

    .manga-list {
        overflow: auto;
        display: flex;
        flex-direction: column;
        align-content: baseline;
        padding: 10px;
    }

    .manga-grid {
        overflow: auto;
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        align-content: baseline;
        padding: 10px;
    }

    .fetch-button {
        color: var(--background-color);
        width: 200px;
        border: none;
        height: 60px;
        margin-left: auto;

        --aug-border-all: 1px;
        --aug-border-bg: var(--accent);
    }

    .fetch-button-enabled {
        background: var(--background-color);
        cursor: pointer;
        color: var(--primary-color);
    }

    .fetch-button-disabled {
        color: var(--primary-color);
        cursor: wait;

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

    .btn {
        background: transparent;
        border: none;
        cursor: pointer;
        margin-left: 20px;
    }

    .label {
        color: var(--primary-color);
    }

    dialog {
        --aug-border-all: 1px;
        --aug-border-bg: var(--accent);

        border-radius: 5px;
        border-width: 1px;
        transition: all 2s;
        width: 90%;
        z-index: 2;
        height: 70%;
        padding: 20px;
        background-color: var(--background-color);
        border: var(--border);

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

    dialog::backdrop {
        background: linear-gradient(rgba(0, 0, 0, 0.4), rgba(0, 0, 0, 0.7));
        animation: fade-in 1s;
    }

    input[type="text"] {
        height: 20px;
        background: transparent;
        color: var(--primary-color);
        border: 1px solid var(--primary-color);
    }

    input[type="text"]:focus-visible {
        outline: none !important;
    }
</style>

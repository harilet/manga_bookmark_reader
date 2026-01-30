<script lang="ts">
    import { open } from "@tauri-apps/plugin-shell";
    import dataStore from "./data_store";
    import Warning from "./icon/warning.svelte";

    export let propValue: any;
    export let mangaListMode: string;

    let causion = false;
    let reTrigger = false;

    $: {
        reTrigger = !reTrigger;
        setClasses();
    }

    async function setClasses() {
        if (await dataStore.get(getUrlWithoutChapter())) {
            causion = true;
        } else {
            causion = false;
        }
    }

    function openUrl() {
        open(propValue["url"]);
    }

    async function setChapterWarning(event: any) {
        event.stopPropagation();

        let url = getUrlWithoutChapter();
        if (await dataStore.get(url)) {
            await dataStore.delete(url);
        } else {
            await dataStore.set(url, true);
        }
        await dataStore.save();
        reTrigger = !reTrigger;
    }

    function getUrlWithoutChapter() {
        return propValue["url"].split("/").slice(0, -1).join("/");
    }

    function getMangaHost() {
        let url = propValue["url"];
        let host = url.split("/")[2];
        host = host.replace("www.", "");
        host = host.split(".")[0];
        return host;
    }
</script>

{#if mangaListMode == "List"}
    <button
        on:click={openUrl}
        class="button-wrappper all-corner-lt-rb {propValue['error']
            ? 'button-wrappper-error'
            : ''}
        {causion ? 'button-wrappper-causion' : ''}"
        data-augmented-ui="tr-clip bl-clip border"
    >
        <div class="main-manga-area font-style">
            <div
                data-augmented-ui="all-hex-alt border"
                class="section-1 {propValue['error'] ? 'index-error' : ''}
            {causion ? 'index-causion' : ''}"
            >
                {propValue["index"]}
            </div>
            <div class="section-2">
                <div class="name">
                    <div>
                        {propValue["name"]}
                    </div>
                    {#if propValue["error"]}
                        <div class="name-error">Error</div>
                    {/if}
                    {#if causion}
                        <div class="name-causion">Causion</div>
                    {/if}
                </div>
                <div style="margin-bottom: 10px;">
                    {getMangaHost()}
                </div>
                <div>
                    <progress
                        class="manga_progress"
                        max={propValue["total_chapters"]}
                        value={propValue["total_chapters"] -
                            propValue["remaining_chapters"]}
                    />
                    {#if propValue["total_chapters"] > 0}
                        {Math.round(
                            ((propValue["total_chapters"] -
                                propValue["remaining_chapters"]) /
                                propValue["total_chapters"]) *
                                100,
                        )}%
                    {/if}
                </div>
            </div>

            <div class="section-3">
                <div class="date-time">
                    {propValue["date_time"]}
                </div>
                <div class="remaining-chapter-text">
                    {propValue["remaining_chapters"]}
                </div>
            </div>
            <div class="section-4">
                <button
                    data-augmented-ui="tr-clip bl-clip border"
                    class="btn"
                    on:click={setChapterWarning}
                >
                    <Warning />
                </button>
            </div>
        </div>
    </button>
{:else if mangaListMode == "Grid"}
    <button
        on:click={openUrl}
        class="grid button-wrappper all-corner-lt-rb {propValue['error']
            ? 'button-wrappper-error'
            : ''}
    {causion ? 'button-wrappper-causion' : ''}"
        data-augmented-ui="tr-clip bl-clip border"
    >
        <div style="width: 100%;">
            <div class="name" style="justify-content: space-between;">
                <div>
                    {propValue["name"]}
                    {#if propValue["error"]}
                        <div
                            class="name-error"
                            style="margin-left: 0px;margin-top: 10px;"
                        >
                            Error
                        </div>
                    {/if}
                    {#if causion}
                        <div
                            class="name-causion"
                            style="margin-left: 0px;margin-top: 10px;"
                        >
                            Causion
                        </div>
                    {/if}
                </div>
                <div
                    style="min-width: 50px;"
                    data-augmented-ui="all-hex-alt border"
                    class="section-1 {propValue['error'] ? 'index-error' : ''}
                {causion ? 'index-causion' : ''}"
                >
                    {propValue["index"]}
                </div>
            </div>
        </div>
        <div>
            <div
                style="margin-bottom: 10px;text-align: initial; display: flex; flex-direction: row; align-items: center; justify-content: space-between;"
            >
                <div>
                    {getMangaHost()}
                </div>
                <div>
                    {propValue["remaining_chapters"]}
                </div>
            </div>
            <div
                style="display: flex; flex-direction: row;align-items: center;"
            >
                <progress
                    class="manga_progress"
                    style="margin-right: 5px;"
                    max={propValue["total_chapters"]}
                    value={propValue["total_chapters"] -
                        propValue["remaining_chapters"]}
                />
                {#if propValue["total_chapters"] > 0}
                    {Math.round(
                        ((propValue["total_chapters"] -
                            propValue["remaining_chapters"]) /
                            propValue["total_chapters"]) *
                            100,
                    )}%
                {/if}
            </div>
            <div style="text-align: initial;">
                {propValue["date_time"]}
            </div>
        </div>
    </button>
{/if}

<style>
    .grid {
        display: flex;
        height: 225px;
        width: 300px;
        flex-direction: row;
        padding: 15px !important;
        margin: 10px !important;
        flex-direction: column;
        justify-content: space-between;
        color: var(--primary-color);
    }
    .btn {
        background: transparent;
        border: none;
        --aug-border-all: 1px;
        --aug-border-bg: var(--accent);
        padding: 8px;
    }

    .section-1 {
        --aug-all-width: 50px;
        --aug-border-all: 1px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .index-error {
        color: red;
        background: #ff000022;
    }

    .name-error {
        color: red;
        background: #ff000022;
        margin-left: 15px;
        padding: 2px;
        border: 1px solid #ff000055;
    }

    .button-wrappper-error {
        --aug-border-bg: #ff000055 !important;
    }

    .index-causion {
        color: yellow;
        background: #ffff0022;
    }

    .name-causion {
        color: yellow;
        background: #ffff0022;
        margin-left: 15px;
        padding: 2px;
        border: 1px solid #ffff0055;
    }

    .button-wrappper-causion {
        --aug-border-bg: #ffff0055 !important;
    }

    .button-wrappper {
        flex-grow: 1;
        padding: 0px;
        border: none;
        --aug-border-all: 1px;
        --aug-border-bg: var(--accent);
        margin-bottom: 10px;
        background: #0f172a99;
    }

    .button-wrappper:hover {
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
            /* panel background */ linear-gradient(180deg, #1e293b99, #1e293b99);

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

    .main-manga-area {
        border-radius: 5px;
        cursor: pointer;
        padding: 10px;
        padding-left: 5px;
        position: relative;
        background: transparent;
        text-align: start;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;
        border-radius: 0px;
        color: var(--primary-color);
        border: var(--border);
    }

    .section-2 {
        display: flex;
        justify-content: space-between;
        flex-direction: column;
        padding-right: 5px;
        width: -webkit-fill-available;
        margin-left: 10px;
    }

    .date-time {
        margin-top: 5px;
        color: gray;
        white-space: nowrap;
    }

    .name {
        margin-bottom: 10px;
        font-weight: 600;
        font-size: 105%;
        width: -webkit-fill-available;
        color: #cefafe;
        display: flex;
        align-items: center;
        flex-direction: row;
        text-align: initial;
    }

    .section-3 {
        display: flex;
        justify-content: space-between;
        flex-direction: column;
        align-items: end;
        font-weight: 600;
    }

    .remaining-chapter-text {
        margin-bottom: 10px;
        color: var(--secondary-text);
    }

    .section-4 {
        display: flex;
        margin-left: 15px;
    }

    .manga_progress {
        width: 100%;
        height: 5px;
        background-color: transparent;
        border: solid 1px var(--primary-color);
    }

    .manga_progress::-webkit-progress-value {
        background: var(--done-color);
    }
    .manga_progress[value]::-webkit-progress-bar {
        background-color: transparent;
    }
</style>

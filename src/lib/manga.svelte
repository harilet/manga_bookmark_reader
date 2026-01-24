<script lang="ts">
    import { open } from "@tauri-apps/plugin-shell";
    import dataStore from "./data_store";
    import ContextMenu from "./contextMenu.svelte";

    export let propValue: any;

    let borderClass = "";
    let reTrigger = false;

    $: {
        reTrigger = !reTrigger;
        setClasses();
    }

    async function setClasses() {
        if (await dataStore.get(getUrlWithoutChapter())) {
            borderClass = "warning-class";
        } else {
            borderClass = "normal-class";
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

<button
    on:click={openUrl}
    class="button-wrappper all-corner-lt-rb"
    data-augmented-ui="tr-clip bl-clip border"
>
    <div class="main-manga-area {borderClass} font-style">
        <div data-augmented-ui="all-hex-alt border" class="section-1">
            {propValue["index"]}
        </div>
        <div class="section-2">
            <div class="name">
                {propValue["name"]}
            </div>
            <div>
                {getMangaHost()}
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
    </div>
</button>

<style>
    .section-1 {
        --aug-all-width: 50px;
        --aug-border-all: 1px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .button-wrappper {
        flex-grow: 1;
        padding: 0px;
        border: none;
        --aug-border-all: 1px;
        --aug-border-bg: var(--accent);
        margin-bottom: 10px;
        background: #0f172a66;
    }

    .button-wrappper:hover {
        background-color: #1e293b99;
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
            /* panel background */ linear-gradient(180deg, #0b1e2e, #081521);

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
    }

    .wrapper-class {
        padding: 2px;
        flex: auto;
    }

    .warning-class {
        color: var(--primary-color);
        border: var(--border);
        text-shadow:
            2px 2px 0 var(--background-color),
            2px -2px 0 var(--background-color),
            -2px 2px 0 var(--background-color),
            -2px -2px 0 var(--background-color),
            2px 0px 0 var(--background-color),
            0px 2px 0 var(--background-color),
            -2px 0px 0 var(--background-color),
            0px -2px 0 var(--background-color);
        background: repeating-linear-gradient(
            90deg,
            var(--primary-color),
            var(--primary-color) 1px,
            var(--background-color) 1px,
            var(--background-color) 3px
        );
    }

    .normal-class {
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
</style>

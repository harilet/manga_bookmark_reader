<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import dataStore from "./data_store";

    export let value = 0;
    export let max = 0;
    export let listCount = 0;

    $: threadCount = 0;
    $: currentThreadCount = 0;
    let progress_bar_class = "";
    let mangaInThreadDialog: HTMLDialogElement;
    $: mangaInThread = [""];

    $: {
        if (value >= max) {
            progress_bar_class = "progress_done";
        } else {
            progress_bar_class = "in_progress";
        }
    }

    onMount(async () => {
        mangaInThread = mangaInThread.filter((x) => x !== "");
        let tempThreadCount = await dataStore.get("thread_count");
        console.log("tempThreadCount", tempThreadCount);
        if (typeof tempThreadCount === "string") {
            threadCount = Number.parseInt(tempThreadCount);
            changeThreadCount();
        } else if (tempThreadCount === null) {
            threadCount = 1;
            changeThreadCount();
        }

        mangaInThreadDialog.addEventListener("click", function (event) {
            var rect = mangaInThreadDialog.getBoundingClientRect();
            var isInDialog =
                rect.top <= event.clientY &&
                event.clientY <= rect.top + rect.height &&
                rect.left <= event.clientX &&
                event.clientX <= rect.left + rect.width;
            if (!isInDialog) {
                mangaInThreadDialog.close();
            }
        });

        await listen("thread_update", (event: { payload: any }) => {
            let data = String(event.payload);
            if (data[0] == "+") {
                mangaInThread = [...mangaInThread, data.substring(1)];
            } else {
                let temp = mangaInThread.filter(function (x) {
                    return x != data.substring(1);
                });
                mangaInThread = [];
                mangaInThread = [...temp];
            }
        });

        await listen("threads", (event) => {
            let data: string[] = String(event.payload).split("/");
            currentThreadCount = parseInt(data[0]);
        });
    });

    function showThreadCountInput() {
        if (mangaInThreadDialog.open) {
            mangaInThreadDialog.close();
        } else {
            mangaInThreadDialog.showModal();
        }
    }

    function changeThreadCount() {
        invoke("update_thread_count", {
            newThreadCount: parseInt(threadCount + ""),
        });
    }

    async function onThreadCountChange(data: any) {
        let dataValue = data.target.value;
        if (dataValue !== "" && dataValue != "0") {
            dataValue = parseInt(dataValue);
            await dataStore.set("thread_count", dataValue + "");
            threadCount = dataValue;
            changeThreadCount();
            mangaInThreadDialog.close();
        }
    }
</script>

<dialog
    data-augmented-ui="tr-clip bl-clip border"
    bind:this={mangaInThreadDialog}
    on:close
>
    <div style="height: 100%;display: flex;flex-direction: column;">
        <div class="settings-header">
            <input
                type="text"
                placeholder="Number of threads"
                value={threadCount}
                on:change={onThreadCountChange}
                style="text-align: center;"
            />
        </div>
        <div class="main-scrollbar" style="overflow: auto;">
            {#key mangaInThread}
                {#each mangaInThread as manga_item}
                    <div
                        style="border: solid 1px var(--primary-color);width: 99%;"
                    >
                        <div class="manga-in-thread-wrapper">
                            <div
                                data-augmented-ui="r-rect l-rect"
                                class="manga-in-thread"
                            >
                                {#if manga_item !== ""}
                                    {manga_item}
                                {/if}
                            </div>
                        </div>
                    </div>
                {/each}
            {/key}
        </div>
    </div>
</dialog>

<div class="progress-area-wrapper">
    <div
        data-augmented-ui="bl-clip t-clip-x br-clip border"
        class="progress-area"
    >
        <progress
            data-augmented-ui="bl-clip t-clip-x br-clip"
            class="progress {progress_bar_class}"
            {value}
            {max}
        >
        </progress>
    </div>
</div>

<div class="progress-test">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div style="display: flex;cursor: pointer;" on:click={showThreadCountInput}>
        {#key threadCount}
            {#each [...Array(threadCount).keys()] as i (i)}
                <div
                    class="thread-box"
                    style="--fill: {currentThreadCount <= i
                        ? 'transparent'
                        : 'var(--primary-color)'};"
                >
                    <span></span>
                </div>
            {/each}
        {/key}
    </div>

    <div>
        {listCount} : {value} of {max}
    </div>
</div>

<style>
    .thread-box {
        flex: 1;
        transition: opacity 0.3s ease;
        background-color: var(--fill);
        border: var(--border);
        width: 10px;
        height: 10px;
        margin: 0px 1px;
    }

    .progress-area-wrapper {
        padding: 0px;
    }

    .progress-area {
        display: flex;
        padding: 4px 2px;
        justify-content: space-evenly;
        --aug-border-all: 1px;
        --aug-border-bg: var(--primary-color);
    }

    .progress-test {
        padding: 5px;
        display: flex;
        align-items: center;
        justify-content: space-between;
        color: var(--primary-color);
    }

    .progress {
        width: 100%;
        border-radius: 0px;
        height: 20px;
    }

    .in_progress::-webkit-progress-value {
        background: var(--in-progress-color);
    }
    .progress_done::-webkit-progress-value {
        background: var(--done-color);
    }
    .progress[value]::-webkit-progress-bar {
        background-color: transparent;
    }

    .thread-progress {
        background: transparent;
        border: var(--border);
    }

    .thread-progress[value]::-webkit-progress-bar {
        background-color: transparent;
    }

    .thread-progress::-webkit-progress-value {
        background: var(--done-color);
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

    .manga-in-thread-wrapper {
        background: var(--primary-color);
        border: var(--border);
    }

    .manga-in-thread {
        background: var(--background-color);
        padding: 20px;
        color: var(--primary-color);
    }

    .settings-header {
        display: flex;
        justify-content: space-between;
    }

    .btn {
        background: transparent;
        border: none;
        cursor: pointer;
        margin-left: 20px;
    }
</style>

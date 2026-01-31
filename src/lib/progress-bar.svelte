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
    $: mangaInThread = [{}];
    let color = "#81a1c1";

    $: {
        const root = document.documentElement;
        root.style.setProperty(`--primary-color`, color);
        if (value >= max) {
            progress_bar_class = "progress_done";
        } else {
            progress_bar_class = "in_progress";
        }
    }

    onMount(async () => {
        mangaInThread = mangaInThread.filter((x: any) => x !== "");
        let tempThreadCount = await dataStore.get("thread_count");
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
            let data = event.payload;
            if (data["added"] == true) {
                mangaInThread = [
                    ...mangaInThread,
                    { url: data["url"], thread: data["thread_num"] },
                ];
            } else {
                let temp = mangaInThread.filter(function (x) {
                    return x["url"] != data["url"];
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

    function isThreadActive(threadNumber: number): boolean {
        let isTActive = mangaInThread.some(
            (item: any) => item.thread === threadNumber,
        );
        return isTActive;
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
            <div style="margin-left: 20px;">
                <input
                    type="color"
                    id="favcolor"
                    name="favcolor"
                    bind:value={color}
                />
            </div>
        </div>
        <div class="main-scrollbar" style="overflow: auto;">
            {#key mangaInThread}
                {#each mangaInThread as manga_item}
                    <div
                        class="manga-in-thread"
                        data-augmented-ui="tr-clip bl-clip border"
                    >
                        <div
                            data-augmented-ui="all-hex-alt border"
                            style="--aug-border-all: 1px;
                        --aug-border-bg: var(--accent); margin-right: 10px; min-width: 30px;"
                        >
                            {manga_item["thread"]}
                        </div>
                        {#if manga_item !== ""}
                            {manga_item["url"]}
                        {/if}
                    </div>
                {/each}
            {/key}
        </div>
    </div>
</dialog>

<div class="progress-area-wrapper">
    <div class="progress-area all-corner-stroke">
        <progress class="progress {progress_bar_class}" {value} {max}>
        </progress>
    </div>
</div>

<div class="progress-test">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div style="display: flex;cursor: pointer;" on:click={showThreadCountInput}>
        {#key mangaInThread}
            {#each [...Array(threadCount).keys()] as i (i)}
                <div
                    class="thread-box"
                    style="--fill: {isThreadActive(i)
                        ? 'var(--accent)'
                        : 'transparent'};"
                >
                    <span>
                        {i + 1}
                    </span>
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
        width: 20px;
        height: 20px;
        margin: 0px 1px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .progress-area-wrapper {
        padding: 0px;
    }

    .progress-area {
        display: flex;
        padding: 5px;
        justify-content: space-evenly;
        border: solid 1px var(--primary-color);
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
        height: 10px;
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

    .manga-in-thread {
        --aug-border-all: 1px;
        --aug-border-bg: var(--accent);
        display: flex;
        align-items: center;

        border: var(--border);
        margin: 5px;
        padding: 20px;
        color: var(--primary-color);
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

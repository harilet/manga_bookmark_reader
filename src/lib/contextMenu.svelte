<script lang="ts">
  export let items: any[] = [];
  export let onClick: any;

  let pos = { x: 0, y: 0 };
  let menu = { h: 0, w: 0 };
  let browser = { h: 0, w: 0 };
  let showMenu = false;

  function rightClickContextMenu(e: { clientX: any; clientY: any }) {
    showMenu = true;
    browser = {
      w: window.innerWidth,
      h: window.innerHeight,
    };
    pos = {
      x: e.clientX,
      y: e.clientY,
    };

    if (browser.h - pos.y < menu.h) pos.y = pos.y - menu.h;
    if (browser.w - pos.x < menu.w) pos.x = pos.x - menu.w;
  }

  function onPageClick(e: any) {
    showMenu = false;
  }

  function onBtnClick(event: any, itemId: string) {
    event.stopPropagation();
    onClick(event, itemId);
    showMenu = false;
  }

  function getKeys(groupItems: any) {
    let keys = Object.keys(groupItems);
    return keys;
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div on:contextmenu|preventDefault={rightClickContextMenu}>
  {#if showMenu}
    <div
      style="position: absolute; top:{pos.y}px; left:{pos.x}px; z-index: 10;"
    >
      <div class="context-menu">
        {#each items as item}
          {#if "Group" in item}
            {#each item["Group"] as groupItems}
              {#each getKeys(groupItems) as groupItemKey}
                {groupItemKey}
                {#each groupItems[groupItemKey] as groupItem}
                  <button
                    class="btn-style"
                    on:click={(e) => {
                      onBtnClick(e, groupItem['id']);
                    }}
                  >
                    {groupItem["name"]}
                  </button>
                {/each}
              {/each}
            {/each}
          {:else}
            <button
              class="btn-style"
              on:click={(e) => {
                onBtnClick(e, item['id']);
              }}
            >
              {item["name"]}
            </button>
          {/if}
        {/each}
      </div>
    </div>
  {/if}
  <slot />
</div>

<svelte:window on:click={onPageClick} />

<style>
  .context-menu {
    display: inline-flex;
    border: 1px #999 solid;
    width: 170px;
    background-color: var(--background-color);
    color: var(--primary-color);
    border-radius: 3px;
    overflow: hidden;
    flex-direction: column;
  }

  .btn-style {
    background-color: var(--background-color);
    color: var(--primary-color);
    border: solid 1px rgb(255 255 255 / 0.1);
    margin: 5px 0px;
    cursor: pointer;
  }
</style>

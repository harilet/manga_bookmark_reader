<script>
  import { getCurrentWindow } from "@tauri-apps/api/window";

  export let selectedTab = "manga";

  const appWindow = getCurrentWindow();

  function onClickReduce() {
    appWindow.minimize();
  }

  function onClickMaximize() {
    appWindow.toggleMaximize();
  }

  function onClickClose() {
    appWindow.close();
  }

  function tabSelect(item) {
    selectedTab = item;
  }
</script>

<div data-tauri-drag-region class="title-bar">
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="tabs">
    <div
      on:click={(_) => tabSelect("manga")}
      class="tab-wrapper"
      style={selectedTab == "manga" ? "" : "background: black;"}
    >
      <div
        class="tab"
        data-augmented-ui="
        bl-clip-x br-clip-x
     "
      >
        manga
      </div>
    </div>
    <div
      on:click={(_) => tabSelect("anime")}
      class="tab-wrapper"
      style={selectedTab == "anime" ? "" : "background: black;"}
    >
      <div
        class="tab"
        data-augmented-ui="
        bl-clip-x br-clip-x
     "
      >
        anime
      </div>
    </div>
  </div>
  <div class="window-controls">
    <button class="hvrBgDark" on:click={onClickReduce}>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 16.933333 16.933334"
        height="13"
        width="13"
      >
        <g>
          <path
            d="M 1.8515625,7.8046875 A 0.66145998,0.66145998 0 0 0 1.1914062,8.4667969 0.66145998,0.66145998 0 0 0 1.8515625,9.1289063 H 15.082031 A 0.66145998,0.66145998 0 0 0 15.742188,8.4667969 0.66145998,0.66145998 0 0 0 15.082031,7.8046875 Z"
            id="path888"
          ></path>
        </g>
      </svg>
    </button>

    <button class="hvrBgDark" on:click={onClickMaximize}>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 16.933333 16.933334"
        height="13"
        width="13"
      >
        <g>
          <path
            id="path841"
            d="M 3.1757812 1.1914062 C 2.087675 1.1914062 1.1914062 2.087675 1.1914062 3.1757812 L 1.1914062 13.757812 C 1.1914068 14.845918 2.0876754 15.742188 3.1757812 15.742188 L 13.757812 15.742188 C 14.845918 15.742187 15.742187 14.845918 15.742188 13.757812 L 15.742188 3.1757812 C 15.742188 2.0876754 14.845918 1.1914068 13.757812 1.1914062 L 3.1757812 1.1914062 z M 3.1757812 2.5136719 L 13.757812 2.5136719 C 14.13096 2.5136721 14.419922 2.8026342 14.419922 3.1757812 L 14.419922 13.757812 C 14.419922 14.130959 14.130959 14.419922 13.757812 14.419922 L 3.1757812 14.419922 C 2.8026342 14.419922 2.5136721 14.13096 2.5136719 13.757812 L 2.5136719 3.1757812 C 2.5136719 2.802634 2.802634 2.5136719 3.1757812 2.5136719 z "
          ></path>
        </g>
      </svg>
    </button>

    <button
      class="btn-close"
      on:click={onClickClose}
      on:keypress={onClickClose}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 16.933333 16.933334"
        height="13"
        width="13"
      >
        <g>
          <path
            id="path839"
            d="M 2.4863281 1.8535156 A 0.66145998 0.66145998 0 0 0 2.0449219 2.0449219 A 0.66145998 0.66145998 0 0 0 2.0449219 2.9804688 L 7.53125 8.4648438 L 2.0449219 13.951172 A 0.66145998 0.66145998 0 0 0 2.0449219 14.886719 A 0.66145998 0.66145998 0 0 0 2.9804688 14.886719 L 8.4648438 9.4023438 L 13.951172 14.886719 A 0.66145998 0.66145998 0 0 0 14.886719 14.886719 A 0.66145998 0.66145998 0 0 0 14.886719 13.951172 L 9.4023438 8.4648438 L 14.886719 2.9804688 A 0.66145998 0.66145998 0 0 0 14.886719 2.0449219 A 0.66145998 0.66145998 0 0 0 13.951172 2.0449219 L 8.4648438 7.53125 L 2.9804688 2.0449219 A 0.66145998 0.66145998 0 0 0 2.4863281 1.8535156 z "
          ></path>
        </g>
      </svg>
    </button>
  </div>
</div>

<style>
  .tab-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 150px;
    background: var(--primary-color);
  }

  .tab {
    background: var(--background-color);
    color: var(--primary-color);
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    border: var(--border);
  }

  .window-controls {
    display: flex;
  }

  .tabs {
    display: flex;
    color: var(--primary-color);
  }

  .title-bar {
    display: flex;
    justify-content: space-between;
    height: 100%;
  }

  .hvrBgDark {
    width: 42px;
    border: none;
    background: transparent;
    display: flex;
    align-items: center;
    justify-content: center;
    fill: var(--primary-color);
  }

  .hvrBgDark:hover {
    background: #8585853b;
  }

  .btn-close {
    width: 45px;
    background: transparent;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    fill: var(--primary-color);
  }

  .btn-close:hover {
    background: rgb(220 38 38);
  }
</style>

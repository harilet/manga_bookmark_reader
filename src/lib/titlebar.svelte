<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";

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
</script>

<div data-tauri-drag-region class="title-bar">
  <div class="brand">
    <span class="brand-mark">M</span>
    <span class="brand-name">manga<b>reader</b></span>
  </div>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="title-slot" data-tauri-drag-region>
    <slot />
  </div>
  <div class="window-controls" data-tauri-drag-region>
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
  .title-bar {
    display: flex;
    align-items: center;
    height: 52px;
    padding-left: 22px;
    border-bottom: 1px solid var(--border-dim);
    background: var(--void);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-right: 22px;
    flex: none;
  }
  .brand-mark {
    width: 26px;
    height: 26px;
    border-radius: 6px;
    background: var(--signal);
    color: #05070c;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--mono);
    font-weight: 700;
    font-size: 13px;
  }
  .brand-name {
    font-family: var(--mono);
    font-size: 12px;
    letter-spacing: 0.22em;
    text-transform: uppercase;
    color: var(--ink);
  }
  .brand-name b {
    color: var(--ink-muted);
    font-weight: 500;
  }

  .title-slot {
    display: flex;
    align-items: center;
    height: 100%;
    flex: 1;
    min-width: 0;
  }

  .window-controls {
    display: flex;
    margin-left: auto;
    height: 100%;
  }

  .hvrBgDark {
    width: 44px;
    border: none;
    background: transparent;
    display: flex;
    align-items: center;
    justify-content: center;
    fill: var(--ink-muted);
    transition:
      background 0.15s ease,
      fill 0.15s ease;
  }

  .hvrBgDark:hover {
    background: rgba(255, 255, 255, 0.06);
    fill: var(--ink);
  }

  .btn-close {
    width: 46px;
    background: transparent;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    fill: var(--ink-muted);
    transition:
      background 0.15s ease,
      fill 0.15s ease;
  }

  .btn-close:hover {
    background: rgb(220 38 38);
    fill: #ffffff;
  }
</style>

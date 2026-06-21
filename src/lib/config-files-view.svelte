<div class="panel-section">
  <div class="section-head">
    <span class="section-label">config files</span>
  </div>
  <div class="config-fields">
    <label>
      <span class="field-label">threads config path</span>
      <input
        type="text"
        id="cfgThreads"
        placeholder="/etc/scraper/threads.json"
      />
    </label>
    <label>
      <span class="field-label">output config path</span>
      <input
        type="text"
        id="cfgOutput"
        placeholder="/var/data/scraper/output.json"
      />
    </label>
  </div>
  <div class="config-save-row">
    <button class="mini-btn" id="saveConfig">save</button>
  </div>
</div>

<style>
  .panel-section .section-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }
  .panel-section .section-label {
    font-family: var(--mono);
    font-size: 10px;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--ink-dim);
  }

  .mini-btn {
    font-family: var(--mono);
    font-size: 11px;
    color: var(--ink-muted);
    background: none;
    border: 1px solid var(--border-dim);
    padding: 5px 11px;
    border-radius: 6px;
    cursor: pointer;
  }
  .mini-btn:hover {
    color: var(--ink);
    border-color: var(--border-mid);
  }

  .config-fields {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 14px;
  }
  .config-fields label {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .field-label {
    font-family: var(--mono);
    font-size: 11px;
    color: var(--ink-muted);
  }
  .config-fields input {
    background: var(--panel-soft);
    border: 1px solid var(--border-dim);
    color: var(--ink);
    font-family: var(--mono);
    font-size: 12px;
    padding: 9px 11px;
    border-radius: 6px;
    outline: none;
  }
  .config-fields input::placeholder {
    color: var(--ink-dim);
  }
  .config-fields input:focus {
    border-color: var(--border-mid);
  }
</style>

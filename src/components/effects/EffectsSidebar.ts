import { editorState } from "../../state/editor";
import { buildEffectGroups, getEffectById } from "../../effects/registry";

export function EffectsSidebar() {
  const groups = buildEffectGroups(editorState.effects, editorState.searchQuery);
  const selected = getEffectById(editorState.effects, editorState.selectedEffectId);

  return `
    <section class="panel effects-panel">
      <div class="panel-header">
        <span class="panel-label">Effects</span>
      </div>
      <div class="effects-search-wrap">
        <input class="effects-search" type="search" placeholder="Search effects..." value="${editorState.searchQuery}" />
      </div>
      <div class="effect-groups">
        ${groups.length ? groups.map((group) => `
          <div class="effect-group ${editorState.expandedCategories.has(group.category) ? "expanded" : ""}">
            <button class="group-toggle" type="button" data-group="${group.category}">
              <span>▾</span>
              <span>${group.category}</span>
            </button>
            <div class="group-items">
              ${group.effects.map((effect) => `
                <button
                  type="button"
                  class="effect-item ${selected?.id === effect.id ? "selected" : ""}"
                  data-effect-id="${effect.id}"
                >
                  <span>${effect.name}</span>
                </button>
              `).join("")}
            </div>
          </div>
        `).join("") : `<div class="empty-filter-state">No effects match your search.</div>`}
      </div>

      <div class="effect-stack">
        <div class="stack-header">
          <span class="panel-label">Effect stack</span>
          <span>${editorState.effectStack.length}</span>
        </div>
        ${editorState.effectStack.length ? editorState.effectStack.map((item) => `
          <div class="stack-row ${item.id === editorState.selectedStackItemId ? "selected" : ""}">
            <button class="stack-select" type="button" data-stack-id="${item.id}">
              <span class="stack-state">${item.enabled ? "On" : "Off"}</span>
              <span>${item.name}</span>
            </button>
            <button class="stack-remove" type="button" data-remove-stack-id="${item.id}" aria-label="Remove ${item.name}">x</button>
          </div>
        `).join("") : `<div class="empty-filter-state">Select an effect to add it here.</div>`}
      </div>

      ${selected ? `
        <div class="effect-detail">
          <div class="detail-header">
            <div>
              <div class="detail-kicker">${selected.category}</div>
              <h3>${selected.name}</h3>
            </div>
          </div>
          <p>${selected.description}</p>
          <div class="parameter-list">
            ${selected.parameters.map((param) => `
              <div class="parameter-row">
                <label>${param.name}</label>
                <div class="parameter-control">
                  <input type="range" data-parameter-id="${param.id}" min="${param.min ?? 0}" max="${param.max ?? 100}" step="${param.step ?? 1}" value="${editorState.selectedEffectValues[param.id] ?? param.default ?? 0}" />
                  <output>${editorState.selectedEffectValues[param.id] ?? param.default ?? 0}</output>
                </div>
              </div>
            `).join("")}
          </div>
          <div class="detail-actions">
            <button type="button" class="secondary" data-effect-action="cancel">Cancel</button>
            <button type="button" class="primary" data-effect-action="apply">Apply</button>
          </div>
        </div>
      ` : ""}
    </section>
  `;
}

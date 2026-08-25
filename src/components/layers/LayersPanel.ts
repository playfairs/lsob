import { defaultLayers } from "../../state/editor";

export function LayersPanel() {
  return `
    <section class="panel layers-panel">
      <div class="panel-header">
        <span class="panel-label">Layers</span>
        <div class="panel-actions">
          <button type="button">+</button>
          <button type="button">−</button>
          <button type="button">⧉</button>
        </div>
      </div>
      <div class="layer-list">
        ${defaultLayers
          .map(
            (layer) => `
              <div class="layer-row ${layer.selected ? "selected" : ""}">
                <span class="layer-visibility">👁</span>
                <span>${layer.name}</span>
              </div>
            `,
          )
          .join("")}
      </div>
    </section>
  `;
}

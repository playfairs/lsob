import { editorState } from "../../state/editor";

export function Canvas() {
  const hasImage = Boolean(editorState.previewUrl);

  return `
    <div class="canvas-panel">
      <div class="canvas-surface">
        <div class="canvas-grid"></div>
        ${hasImage
          ? `<img class="canvas-image" draggable="false" src="${editorState.previewUrl}" alt="${editorState.fileName || "Editor preview"}" />`
          : `<div class="canvas-placeholder"><div class="canvas-empty">Open an image to begin editing</div></div>`}
      </div>
      <div class="canvas-controls">
        <button type="button" data-canvas-action="fit">Fit</button>
        <button type="button" data-canvas-action="reset">100%</button>
        <button type="button" data-canvas-action="zoom-in">Zoom In</button>
        <button type="button" data-canvas-action="zoom-out">Zoom Out</button>
      </div>
    </div>
  `;
}

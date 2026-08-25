import { editorState, tools } from "../../state/editor";

export function Toolbar() {
  return `
    <div class="toolbar" aria-label="Tool palette">
      ${tools
        .map(
          (tool) => `
            <button
              class="tool-button ${editorState.activeTool === tool.id ? "selected" : ""}"
              type="button"
              data-tool-id="${tool.id}"
              title="${tool.label} (${tool.hotkey ?? ""})"
            >
              <span class="tool-icon">${tool.icon}</span>
            </button>
          `,
        )
        .join("")}
    </div>
  `;
}

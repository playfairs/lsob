import { editorState, tools } from "../../state/editor";

export function ToolSidebar() {
  return `
    <aside class="tool-sidebar">
      <div class="panel-header">
        <span class="panel-label">Tools</span>
      </div>
      <div class="tool-list">
        ${tools
          .map(
            (tool) => `
              <button
                class="tool-item ${editorState.activeTool === tool.id ? "selected" : ""}"
                type="button"
                data-tool-id="${tool.id}"
              >
                <span class="tool-ico">${tool.icon}</span>
                <span>${tool.label}</span>
              </button>
            `,
          )
          .join("")}
      </div>
    </aside>
  `;
}

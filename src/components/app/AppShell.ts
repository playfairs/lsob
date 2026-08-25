import { MenuBar } from "./MenuBar";
import { StatusBar } from "./StatusBar";
import { Canvas } from "../canvas/Canvas";
import { EffectsSidebar } from "../effects/EffectsSidebar";
import { editorState } from "../../state/editor";

export function AppShell() {
  return `
    <div class="app-shell">
      ${MenuBar()}
      <div class="editor-shell">
        <main class="workbench">
          ${Canvas()}
        </main>
        ${editorState.rightSidebarOpen ? `<aside class="right-dock">${EffectsSidebar()}</aside>` : ""}
      </div>
      ${StatusBar()}
    </div>
  `;
}

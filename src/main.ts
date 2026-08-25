import "./styles.css";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { AppShell } from "./components/app/AppShell";
import { loadEffectCatalog } from "./effects/registry";
import {
  editorState,
  pushEffectToStack,
  resetSelectedEffectValues,
  selectEffectStackItem,
  updateSelectedEffectValue,
} from "./state/editor";

const app = document.querySelector<HTMLDivElement>("#app");
let moveStart: { x: number; y: number; offsetX: number; offsetY: number } | null = null;
let previewRequest = 0;
let previewTimer: ReturnType<typeof setTimeout> | undefined;

async function bootstrap() {
  if (!app) return;

  await setupNativeFileDrop();

  const effects = await loadEffectCatalog();
  editorState.effects = effects;
  editorState.categories = Array.from(new Set(effects.map((effect) => effect.category)));
  editorState.expandedCategories = new Set();

  if (effects[0]) {
    resetSelectedEffectValues(effects[0]);
  }

  render();
}

function render() {
  if (!app) return;
  app.innerHTML = AppShell();
  bindUI();
}

function bindUI() {
  if (!app) return;

  if (!app.dataset.bound) {
    app.addEventListener("click", (event) => {
      const target = event.target as HTMLElement | null;
      if (!target) return;

      const menuButton = target.closest(".menu-button");
      if (menuButton) {
        const group = menuButton.closest(".menu-group");
        if (!group) return;
        const isOpen = group.classList.contains("open");
        document.querySelectorAll(".menu-group").forEach((item) => item.classList.remove("open"));
        if (!isOpen) group.classList.add("open");
        return;
      }

      const menuItem = target.closest(".menu-item");
      if (menuItem) {
        const item = menuItem.getAttribute("data-menu-item");
        if (item === "Open") triggerImagePicker();
        if (item === "Export") exportCurrentImage();
        document.querySelectorAll(".menu-group").forEach((group) => group.classList.remove("open"));
        return;
      }

      const effectButton = target.closest("[data-effect-id]");
      if (effectButton) {
        const id = effectButton.getAttribute("data-effect-id");
        if (!id) return;
        editorState.selectedEffectId = id;
        const effect = editorState.effects.find((item) => item.id === id);
        if (effect) {
          resetSelectedEffectValues(effect);
          pushEffectToStack(effect);
        }
        render();
        void renderPreview();
        return;
      }

      const stackButton = target.closest("[data-stack-id]");
      if (stackButton) {
        const stackId = Number(stackButton.getAttribute("data-stack-id"));
        if (Number.isFinite(stackId)) selectEffectStackItem(stackId);
        render();
        return;
      }

      const groupButton = target.closest("[data-group]");
      if (groupButton) {
        const category = groupButton.getAttribute("data-group");
        if (!category) return;
        if (editorState.expandedCategories.has(category)) {
          editorState.expandedCategories.delete(category);
        } else {
          editorState.expandedCategories.add(category);
        }
        render();
        return;
      }

      const canvasButton = target.closest("[data-canvas-action]");
      if (canvasButton) {
        const action = canvasButton.getAttribute("data-canvas-action");
        if (action === "zoom-in") editorState.zoom = Math.min(200, editorState.zoom + 10);
        if (action === "zoom-out") editorState.zoom = Math.max(25, editorState.zoom - 10);
        if (action === "reset") editorState.zoom = 100;
        if (action === "fit") editorState.zoom = 100;
        render();
        return;
      }

      const effectAction = target.closest("[data-effect-action]");
      if (effectAction) {
        const action = effectAction.getAttribute("data-effect-action");
        if (action === "apply") {
          void renderPreview();
          return;
        }
        if (action === "cancel") {
          const stackId = editorState.selectedStackItemId;
          editorState.effectStack = editorState.effectStack.filter((item) => item.id !== stackId);
          const next = editorState.effectStack.at(-1);
          if (next) {
            selectEffectStackItem(next.id);
          } else {
            editorState.selectedStackItemId = 0;
            editorState.selectedEffectId = "";
            editorState.selectedEffectValues = {};
          }
          render();
          void renderPreview();
        }
        return;
      }

      const removeButton = target.closest("[data-remove-stack-id]");
      if (removeButton) {
        const stackId = Number(removeButton.getAttribute("data-remove-stack-id"));
        const removingSelected = editorState.selectedStackItemId === stackId;
        editorState.effectStack = editorState.effectStack.filter((item) => item.id !== stackId);
        if (removingSelected) {
          const next = editorState.effectStack.at(-1);
          if (next) {
            selectEffectStackItem(next.id);
          } else {
            editorState.selectedStackItemId = 0;
            editorState.selectedEffectId = "";
            editorState.selectedEffectValues = {};
          }
        }
        render();
        void renderPreview();
        return;
      }

      if (!target.closest(".menu-group") && !target.closest(".menu-button")) {
        document.querySelectorAll(".menu-group").forEach((group) => group.classList.remove("open"));
      }
    });

    app.addEventListener("input", (event) => {
      const target = event.target as HTMLInputElement;
      if (target.matches(".effects-search")) {
        editorState.searchQuery = target.value;
        render();
        return;
      }
      if (!target.matches("[data-parameter-id]")) return;
      updateSelectedEffectValue(target.dataset.parameterId ?? "", Number(target.value));
      const output = target.parentElement?.querySelector("output");
      if (output) output.textContent = target.value;
      schedulePreview();
    });

    app.addEventListener("dragover", (event) => {
      if ((event.target as HTMLElement).closest(".canvas-surface")) event.preventDefault();
    });

    app.addEventListener("drop", (event) => {
      const target = event.target as HTMLElement;
      if (!target.closest(".canvas-surface")) return;
      event.preventDefault();
      const file = event.dataTransfer?.files[0];
      if (file?.type.startsWith("image/")) loadImageFile(file);
    });

    app.dataset.bound = "true";
  }
}

function triggerImagePicker() {
  void open({
    multiple: false,
    directory: false,
    filters: [{ name: "Images", extensions: ["png", "jpg", "jpeg", "gif", "webp", "bmp", "tiff"] }],
  }).then((path) => {
     if (typeof path === "string") void loadImagePath(path);
  });
}

function loadImageFile(file: File) {
  if (editorState.previewUrl.startsWith("blob:")) URL.revokeObjectURL(editorState.previewUrl);
  editorState.fileName = file.name;
  editorState.previewUrl = URL.createObjectURL(file);
  editorState.originalPreviewUrl = editorState.previewUrl;
  editorState.effectStack = [];
  editorState.selectedStackItemId = 0;
  editorState.imageOffsetX = 0;
  editorState.imageOffsetY = 0;
  editorState.imageScale = 1;
  editorState.imageRotation = 0;
  render();
}

async function loadImagePath(path: string) {
  editorState.fileName = path.split(/[\\/]/).pop() || "Untitled image";
  try {
    editorState.previewUrl = await invoke<string>("load_image", { path });
  } catch {
    editorState.previewUrl = convertFileSrc(path);
  }
  editorState.originalPreviewUrl = editorState.previewUrl;
  editorState.effectStack = [];
  editorState.selectedStackItemId = 0;
  editorState.imageOffsetX = 0;
  editorState.imageOffsetY = 0;
  editorState.imageScale = 1;
  editorState.imageRotation = 0;
  render();
}

async function renderPreview(repaint = true) {
  const request = ++previewRequest;
  if (!editorState.originalPreviewUrl) return;
  if (!editorState.effectStack.length) {
    editorState.previewUrl = editorState.originalPreviewUrl;
    if (!repaint) {
      const image = app?.querySelector<HTMLImageElement>("[data-image-target]");
      if (image) image.src = editorState.previewUrl;
    }
    return;
  }

  try {
    const response = await fetch(editorState.originalPreviewUrl);
    const bytes = Array.from(new Uint8Array(await response.arrayBuffer()));
    const effects = editorState.effectStack.map((item) => ({
      kind: item.effectId,
      value: Number(Object.values(item.values)[0] ?? 0),
      enabled: item.enabled,
    }));
    const previewUrl = await invoke<string>("preview_image", { bytes, effects });
    if (request !== previewRequest) return;
    editorState.previewUrl = previewUrl;
    if (repaint) {
      render();
    } else {
      const image = app?.querySelector<HTMLImageElement>("[data-image-target]");
      if (image) image.src = previewUrl;
    }
  } catch {
    if (request !== previewRequest) return;
    editorState.previewUrl = editorState.originalPreviewUrl;
  }
}

function schedulePreview() {
  if (previewTimer) clearTimeout(previewTimer);
  previewTimer = setTimeout(() => {
    previewTimer = undefined;
    void renderPreview(false);
  }, 50);
}

async function setupNativeFileDrop() {
  try {
    await getCurrentWindow().onDragDropEvent((event) => {
      if (event.payload.type !== "drop") return;
      const imagePath = event.payload.paths.find((path) => /\.(png|jpe?g|gif|webp|bmp|tiff?)$/i.test(path));
      if (imagePath) loadImagePath(imagePath);
    });
  } catch {
  }
}

function exportCurrentImage() {
  if (!editorState.previewUrl) return;
  const link = document.createElement("a");
  link.href = editorState.previewUrl;
  link.download = editorState.fileName || "lsob-export.png";
  link.click();
}

void bootstrap();

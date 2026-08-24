import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import "./styles.css";

type EffectKind =
  | "blur"
  | "pixelate"
  | "brightness"
  | "contrast"
  | "hue"
  | "sharpen"
  | "noise"
  | "rgb"
  | "radial"
  | "melt"
  | "glitch"
  | "finish";
type Effect = { id: number; kind: EffectKind; value: number; enabled: boolean };

const state = {
  bytes: [] as number[],
  previewBytes: [] as number[],
  fileName: "",
  preview: "",
  previewReady: false,
  effects: [] as Effect[],
  collapsed: new Set<number>(),
  nextId: 1,
  renderToken: 0,
  previewRunning: false,
  previewPending: false,
};
const app = document.querySelector<HTMLDivElement>("#app")!;

function effectLabel(kind: EffectKind) {
  return {
    blur: "Gaussian Blur",
    pixelate: "Pixelation",
    brightness: "Brightness",
    contrast: "Contrast",
    hue: "Hue Shift",
    sharpen: "Sharpen",
    noise: "Noise",
    rgb: "RGB Shift",
    radial: "Radial Blur",
    melt: "Melt",
    glitch: "Glitch",
    finish: "Finish",
  }[kind];
}

function effectDefault(kind: EffectKind) {
  return {
    blur: 8,
    pixelate: 10,
    brightness: 0,
    contrast: 0,
    hue: 0,
    sharpen: 2,
    noise: 18,
    rgb: 8,
    radial: 28,
    melt: 20,
    glitch: 25,
    finish: 86,
  }[kind];
}

function effectUnit(kind: EffectKind) {
  return kind === "blur" || kind === "pixelate" || kind === "rgb"
    ? "px"
    : kind === "hue"
      ? "°"
      : kind === "radial" || kind === "melt" || kind === "glitch"
        ? "%"
        : "%";
}

function render() {
  app.innerHTML = `
    <main class="shell">
      <header class="topbar">
        <div class="brand"><span class="brand-mark">l_</span><span>SOB</span><small>IMAGE DESTRUCTION LAB</small></div>
        <div class="top-actions"><label class="open-button">Open image<input id="file-input" type="file" accept="image/png,image/jpeg,image/webp,image/gif,image/bmp,image/tiff"></label><button id="export" class="accent" ${state.previewReady ? "" : "disabled"}>Export PNG</button></div>
      </header>
      <section class="workspace">
        <div class="stage ${state.preview ? "has-image" : ""}">
          ${state.preview ? `<img src="${state.preview}" alt="Live preview">` : `<div class="empty"><div class="empty-icon">+</div><strong>Drop an image here</strong><span>PNG, WebP, JPEG, GIF, BMP or TIFF</span><label class="drop-button">Choose file<input id="empty-input" type="file" accept="image/*"></label></div>`}
          ${state.bytes.length ? `<div class="stage-meta"><span>${state.fileName}</span><span>LIVE PREVIEW · 512 PX</span></div>` : ""}
        </div>
        <aside class="inspector">
          <div class="inspector-head"><div><span class="eyebrow">PROCESS</span><h1>Effect stack</h1></div><div class="stack-tools"><button data-collapse-all aria-label="Collapse all effects">−</button><button data-expand-all aria-label="Expand all effects">+</button><span class="count">${state.effects.length}</span></div></div>
          <div id="effects" class="effects">${state.effects.length ? state.effects.map((effect, index) => effectRow(effect, index)).join("") : `<div class="stack-empty">Add an effect to start destroying clarity.</div>`}</div>
          <div class="add-effect"><span class="eyebrow">ADD EFFECT</span><div class="effect-buttons"><button class="finish-button" data-add="finish">Finish</button><button data-add="blur">Blur</button><button data-add="radial">Radial</button><button data-add="pixelate">Pixelate</button><button data-add="sharpen">Sharpen</button><button data-add="melt">Melt</button><button data-add="glitch">Glitch</button><button data-add="rgb">RGB</button><button data-add="noise">Noise</button><button data-add="brightness">Light</button><button data-add="contrast">Contrast</button><button data-add="hue">Hue</button></div></div>
          <div class="status">${state.bytes.length ? "Preview updates as you work" : "Waiting for an image"}</div>
        </aside>
      </section>
    </main>`;
  bind();
}

function effectRow(effect: Effect, index: number) {
  const ranges = {
    blur: [0, 32],
    pixelate: [1, 64],
    brightness: [-100, 100],
    contrast: [-100, 100],
    hue: [-180, 180],
    sharpen: [0, 8],
    noise: [0, 80],
    rgb: [0, 40],
    radial: [0, 100],
    melt: [0, 80],
    glitch: [0, 80],
    finish: [0, 100],
  }[effect.kind];
  const isCollapsed = state.collapsed.has(effect.id);
  return `<article class="effect-row ${effect.enabled ? "" : "muted"} ${isCollapsed ? "collapsed" : ""}"><div class="effect-title"><button class="toggle ${effect.enabled ? "on" : ""}" data-toggle="${effect.id}">${effect.enabled ? "●" : "○"}</button><button class="effect-summary" data-collapse="${effect.id}"><strong>${effectLabel(effect.kind)}</strong><span class="summary-value">${effect.value}${effectUnit(effect.kind)}</span><span class="chevron">${isCollapsed ? "⌄" : "⌃"}</span></button><button class="icon" data-remove="${effect.id}" aria-label="Remove effect">×</button></div>${isCollapsed ? "" : `<input class="range" data-value="${effect.id}" type="range" min="${ranges[0]}" max="${ranges[1]}" step="${effect.kind === "blur" || effect.kind === "sharpen" ? "0.5" : "1"}" value="${effect.value}"><div class="row-actions"><span>STACK ${String(index + 1).padStart(2, "0")}</span><div><button data-up="${index}" ${index === 0 ? "disabled" : ""}>↑</button><button data-down="${index}" ${index === state.effects.length - 1 ? "disabled" : ""}>↓</button></div></div>`}</article>`;
}

function bind() {
  document
    .querySelector<HTMLInputElement>("#file-input")
    ?.addEventListener("change", (e) =>
      loadFile((e.target as HTMLInputElement).files?.[0]),
    );
  document
    .querySelector<HTMLInputElement>("#empty-input")
    ?.addEventListener("change", (e) =>
      loadFile((e.target as HTMLInputElement).files?.[0]),
    );
  document
    .querySelector<HTMLButtonElement>("#export")
    ?.addEventListener("click", exportImage);
  document
    .querySelector<HTMLButtonElement>("[data-collapse-all]")
    ?.addEventListener("click", () => {
      state.effects.forEach((effect) => state.collapsed.add(effect.id));
      render();
    });
  document
    .querySelector<HTMLButtonElement>("[data-expand-all]")
    ?.addEventListener("click", () => {
      state.collapsed.clear();
      render();
    });
  document
    .querySelectorAll<HTMLButtonElement>("[data-collapse]")
    .forEach((button) =>
      button.addEventListener("click", () => {
        const id = Number(button.dataset.collapse);
        if (state.collapsed.has(id)) state.collapsed.delete(id);
        else state.collapsed.add(id);
        render();
      }),
    );
  document.querySelectorAll<HTMLButtonElement>("[data-add]").forEach((button) =>
    button.addEventListener("click", () => {
      const kind = button.dataset.add as EffectKind;
      const effect = {
        id: state.nextId++,
        kind,
        value: effectDefault(kind),
        enabled: true,
      };
      state.effects.push(effect);
      state.collapsed.delete(effect.id);
      render();
      schedulePreview();
    }),
  );
  document
    .querySelectorAll<HTMLButtonElement>("[data-toggle]")
    .forEach((button) =>
      button.addEventListener("click", () => {
        const effect = state.effects.find(
          (item) => item.id === Number(button.dataset.toggle),
        );
        if (effect) effect.enabled = !effect.enabled;
        render();
        schedulePreview();
      }),
    );
  document
    .querySelectorAll<HTMLButtonElement>("[data-remove]")
    .forEach((button) =>
      button.addEventListener("click", () => {
        state.effects = state.effects.filter(
          (item) => item.id !== Number(button.dataset.remove),
        );
        render();
        schedulePreview();
      }),
    );
  document.querySelectorAll<HTMLInputElement>("[data-value]").forEach((input) =>
    input.addEventListener("input", () => {
      const effect = state.effects.find(
        (item) => item.id === Number(input.dataset.value),
      );
      if (effect) {
        effect.value = Number(input.value);
        const value = input
          .closest(".effect-row")
          ?.querySelector(".summary-value");
        if (value)
          value.textContent = `${effect.value}${effectUnit(effect.kind)}`;
        schedulePreview();
      }
    }),
  );
  document
    .querySelectorAll<HTMLButtonElement>("[data-up]")
    .forEach((button) =>
      button.addEventListener("click", () =>
        move(Number(button.dataset.up), -1),
      ),
    );
  document
    .querySelectorAll<HTMLButtonElement>("[data-down]")
    .forEach((button) =>
      button.addEventListener("click", () =>
        move(Number(button.dataset.down), 1),
      ),
    );
  const stage = document.querySelector(".stage");
  stage?.addEventListener("dragover", (event) => {
    event.preventDefault();
    stage.classList.add("dragging");
  });
  stage?.addEventListener("dragleave", () =>
    stage.classList.remove("dragging"),
  );
  stage?.addEventListener("drop", (event) => {
    const drop = event as DragEvent;
    drop.preventDefault();
    stage.classList.remove("dragging");
    loadFile(drop.dataTransfer?.files[0]);
  });
}

function move(index: number, direction: number) {
  const next = index + direction;
  if (next < 0 || next >= state.effects.length) return;
  [state.effects[index], state.effects[next]] = [
    state.effects[next],
    state.effects[index],
  ];
  render();
  schedulePreview();
}

async function loadFile(file?: File) {
  if (!file) return;
  state.bytes = Array.from(new Uint8Array(await file.arrayBuffer()));
  state.previewBytes = await createPreviewBytes(file);
  state.fileName = file.name;
  state.preview = URL.createObjectURL(file);
  render();
  await schedulePreview();
}

async function createPreviewBytes(file: File): Promise<number[]> {
  const bitmap = await createImageBitmap(file);
  const scale = Math.min(1, 512 / Math.max(bitmap.width, bitmap.height));
  const canvas = document.createElement("canvas");
  canvas.width = Math.max(1, Math.round(bitmap.width * scale));
  canvas.height = Math.max(1, Math.round(bitmap.height * scale));
  canvas.getContext("2d")!.drawImage(bitmap, 0, 0, canvas.width, canvas.height);
  bitmap.close();
  const blob = await new Promise<Blob>((resolve, reject) =>
    canvas.toBlob(
      (result) =>
        result
          ? resolve(result)
          : reject(new Error("Could not create preview")),
      "image/png",
    ),
  );
  return Array.from(new Uint8Array(await blob.arrayBuffer()));
}

let previewTimer: number | undefined;
function schedulePreview() {
  state.previewReady = false;
  document
    .querySelector<HTMLButtonElement>("#export")
    ?.setAttribute("disabled", "true");
  window.clearTimeout(previewTimer);
  previewTimer = window.setTimeout(() => {
    void renderPreview();
  }, 30);
}

async function renderPreview() {
  if (!state.previewBytes.length) return;
  if (state.previewRunning) {
    state.previewPending = true;
    return;
  }
  state.previewRunning = true;
  do {
    state.previewPending = false;
    const token = ++state.renderToken;
    const preview = await invoke<string>("preview_image", {
      bytes: state.previewBytes,
      effects: state.effects,
    });
    if (token === state.renderToken) {
      state.preview = preview;
      state.previewReady = true;
      const image = document.querySelector<HTMLImageElement>(".stage img");
      if (image) image.src = preview;
      document
        .querySelector<HTMLButtonElement>("#export")
        ?.removeAttribute("disabled");
    }
  } while (state.previewPending);
  state.previewRunning = false;
}

async function exportImage() {
  if (!state.previewReady) return;
  const outputPath = await save({
    defaultPath: `lsob-${state.fileName || "destroyed"}.png`,
    filters: [{ name: "PNG image", extensions: ["png"] }],
  });
  if (!outputPath) return;
  try {
    await invoke("save_preview", { dataUrl: state.preview, outputPath });
  } catch (error) {
    window.alert(`Could not export image: ${String(error)}`);
  }
}

render();

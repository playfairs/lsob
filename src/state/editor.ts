import type {
  EffectDefinition,
  EffectParameterDefinition,
  EffectParameterValue,
  EffectStackItem,
} from "../types/effects";

export interface ToolDefinition {
  id: string;
  label: string;
  icon: string;
  hotkey?: string;
}

export const tools: ToolDefinition[] = [
  { id: "move", label: "Move", icon: "✥", hotkey: "V" },
  { id: "crop", label: "Crop", icon: "◫", hotkey: "C" },
  { id: "scale", label: "Scale", icon: "↔", hotkey: "S" },
  { id: "rotate", label: "Rotate", icon: "⟲", hotkey: "R" },
  { id: "transform", label: "Transform", icon: "✦", hotkey: "T" },
  { id: "brush", label: "Brush", icon: "✎", hotkey: "B" },
  { id: "eraser", label: "Eraser", icon: "⌫", hotkey: "E" },
  { id: "clone", label: "Clone", icon: "⧉", hotkey: "Q" },
  { id: "smudge", label: "Smudge", icon: "◌", hotkey: "M" },
  { id: "gradient", label: "Gradient", icon: "▤", hotkey: "G" },
  { id: "fill", label: "Fill", icon: "▣", hotkey: "F" },
  { id: "text", label: "Text", icon: "T", hotkey: "T" },
  { id: "picker", label: "Color Picker", icon: "◉", hotkey: "I" },
];

export const defaultLayers = [
  { id: 1, name: "Background", visible: true, selected: true },
  { id: 2, name: "Layer 1", visible: true, selected: false },
  { id: 3, name: "Layer 2", visible: true, selected: false },
];

export const editorState = {
  activeTool: "move",
  leftSidebarOpen: true,
  rightSidebarOpen: true,
  searchQuery: "",
  categories: [] as string[],
  effects: [] as EffectDefinition[],
  selectedEffectId: "",
  selectedStackItemId: 0,
  selectedEffectValues: {} as Record<string, string | number | boolean>,
  effectStack: [] as EffectStackItem[],
  previewUrl: "",
  originalPreviewUrl: "",
  showOriginal: false,
  zoom: 100,
  imageOffsetX: 0,
  imageOffsetY: 0,
  imageScale: 1,
  imageRotation: 0,
  fileName: "",
  previewReady: false,
  expandedCategories: new Set<string>(),
};

export function getDefaultParameterValue(param: EffectParameterDefinition) {
  if (param.default !== undefined) return param.default;
  if (param.type === "boolean") return false;
  if (param.type === "enum" && param.options?.length)
    return param.options[0].value;
  if (
    param.type === "number" ||
    param.type === "integer" ||
    param.type === "angle" ||
    param.type === "percentage"
  ) {
    return param.min ?? 0;
  }
  return 0;
}

export function getSelectedEffect() {
  return editorState.effects.find(
    (effect) => effect.id === editorState.selectedEffectId,
  );
}

export function resetSelectedEffectValues(effect?: EffectDefinition) {
  const active = effect ?? getSelectedEffect();
  if (!active) return;

  editorState.selectedEffectValues = {};
  active.parameters.forEach((param) => {
    editorState.selectedEffectValues[param.id] =
      getDefaultParameterValue(param);
  });
}

export function pushEffectToStack(effect: EffectDefinition) {
  const existing = editorState.effectStack.find(
    (item) => item.effectId === effect.id,
  );
  if (existing) {
    editorState.selectedStackItemId = existing.id;
    editorState.selectedEffectId = existing.effectId;
    editorState.selectedEffectValues = { ...existing.values };
    return existing;
  }

  const nextId = editorState.effectStack.length + 1;
  const values = { ...editorState.selectedEffectValues };
  const item = {
    id: nextId,
    effectId: effect.id,
    name: effect.name,
    values,
    enabled: true,
  };

  editorState.effectStack.push(item);
  editorState.selectedStackItemId = item.id;
  return item;
}

export function selectEffectStackItem(itemId: number) {
  const item = editorState.effectStack.find(
    (stackItem) => stackItem.id === itemId,
  );
  if (!item) return;
  editorState.selectedStackItemId = item.id;
  editorState.selectedEffectId = item.effectId;
  editorState.selectedEffectValues = { ...item.values };
}

export function updateSelectedEffectValue(
  parameterId: string,
  value: EffectParameterValue,
) {
  editorState.selectedEffectValues[parameterId] = value;
  const item = editorState.effectStack.find(
    (stackItem) => stackItem.id === editorState.selectedStackItemId,
  );
  if (item) item.values[parameterId] = value;
}

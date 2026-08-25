import { invoke } from "@tauri-apps/api/core";
import type { EffectDefinition } from "../types/effects";

export async function loadEffectCatalog(): Promise<EffectDefinition[]> {
  return invoke<EffectDefinition[]>("get_effects");
}

export async function loadEffectCategories(): Promise<string[]> {
  return invoke<string[]>("get_effect_categories");
}

export function buildEffectGroups(effects: EffectDefinition[], query = "") {
  const normalized = query.trim().toLowerCase();

  const filtered = effects.filter((effect) => {
    if (!normalized) return true;

    const haystacks = [
      effect.name,
      effect.category,
      effect.description,
      ...(effect.aliases ?? []),
    ].join(" ").toLowerCase();

    return haystacks.includes(normalized);
  });

  const groups = new Map<string, EffectDefinition[]>();
  filtered.forEach((effect) => {
    const group = groups.get(effect.category) ?? [];
    group.push(effect);
    groups.set(effect.category, group);
  });

  return Array.from(groups.entries()).map(([category, items]) => ({
    category,
    effects: items.sort((a, b) => a.name.localeCompare(b.name)),
  }));
}

export function getEffectById(
  effects: EffectDefinition[],
  id: string,
): EffectDefinition | undefined {
  return effects.find((effect) => effect.id === id);
}

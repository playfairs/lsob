export type EffectParameterType =
  | "number"
  | "integer"
  | "boolean"
  | "enum"
  | "color"
  | "angle"
  | "percentage"
  | "vector2"
  | "curve"
  | "text";

export type EffectParameterValue = string | number | boolean;

export interface EffectOption {
  label: string;
  value: string;
}

export interface EffectParameterDefinition {
  id: string;
  name: string;
  type: EffectParameterType;
  default?: EffectParameterValue;
  min?: number;
  max?: number;
  step?: number;
  unit?: string;
  options?: EffectOption[];
}

export interface EffectDefinition {
  id: string;
  name: string;
  category: string;
  description: string;
  aliases: string[];
  supportsPreview: boolean;
  parameters: EffectParameterDefinition[];
}

export interface EffectStackItem {
  id: number;
  effectId: string;
  name: string;
  values: Record<string, EffectParameterValue>;
  enabled: boolean;
}

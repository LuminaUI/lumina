export type LuminaColorScheme = 'light' | 'dark';
export interface LuminaColorSchemeManager {
  get: (defaultValue: LuminaColorScheme) => LuminaColorScheme;
  set: (value: LuminaColorScheme) => void;
  subscribe(callback: (colorScheme: LuminaColorScheme) => void): () => void;
  clear: () => void;
}

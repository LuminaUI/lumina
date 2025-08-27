/**
 * Sets up "derive" relationships between a parent StyleSheet and one or more base StyleSheets.
 *
 * In RSML/Roblox styles, a StyleSheet can *derive* from other sheets, meaning
 * it inherits their definitions (tokens, styles, etc.) while still allowing
 * local overrides. This is useful for layering styles, such as applying a
 * component’s styles on top of a theme, or merging multiple style sources.
 *
 * This helper ensures that the given `parent` StyleSheet derives from all
 * provided `children` StyleSheets. Derives are applied in the order given:
 * earlier children take precedence over later ones when resolving conflicts.
 *
 * @param parent - The StyleSheet that should derive values (e.g. a component stylesheet).
 * @param children - One or more StyleSheets for the parent to inherit from
 *                   (e.g. theme, base tokens, overrides).
 *
 * @example
 * ```ts
 * const scheme = useColorScheme();
 *
 * const themeSheet = scheme.scheme === "dark"
 *   ? darkStyleSheet
 *   : lightStyleSheet;
 *
 * const baseTokens = tokensStyleSheet;
 *
 * // Link the component stylesheet to both the theme and token sheets
 * set_derives(componentStyleSheet, baseTokens, themeSheet);
 * ```
 *
 * After this call, `componentStyleSheet` will resolve values by checking:
 *   1. Its own definitions
 *   2. `baseTokens`
 *   3. `themeSheet`
 * in that order, falling back as needed.
 */
export function set_derives(parent: StyleSheet, ...children: StyleSheet[]) {
	parent.SetDerives(children);
}

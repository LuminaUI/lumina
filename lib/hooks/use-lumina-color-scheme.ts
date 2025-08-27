import { useCallback, useContext } from '@rbxts/react';
import { LuminaContext } from '../core/LuminaContext';
import { LuminaColorScheme } from '../theme';

export function useColorScheme() {
	const ctx = useContext(LuminaContext);

	assert(ctx, 'LuminaProvider not found in node tree');

	const set = useCallback(
		(value: LuminaColorScheme) => ctx.setColorScheme(value),
		[ctx],
	);
	const clear = useCallback(() => ctx.clearColorScheme(), [ctx]);

	const toggle = useCallback(
		() => set(ctx.colorScheme === 'light' ? 'dark' : 'light'),
		[set, ctx.colorScheme],
	);

	return {
		scheme: ctx.colorScheme,
		set,
		toggle,
		clear,
	};
}

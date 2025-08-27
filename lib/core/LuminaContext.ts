import { createContext, useContext } from '@rbxts/react';
import { LuminaColorScheme } from '../theme';

interface LuminaContextValue {
	colorScheme: LuminaColorScheme;
	setColorScheme: (colorScheme: LuminaColorScheme) => void;
	clearColorScheme: () => void;
}

export const LuminaContext = createContext<LuminaContextValue | undefined>(
	undefined,
);

export function useLuminaContext() {
	const ctx = useContext(LuminaContext);

	assert(ctx, 'LuminaProvider was not found in node tree');

	return ctx;
}

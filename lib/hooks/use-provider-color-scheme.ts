import { useCallback, useEffect, useState } from '@rbxts/react';
import { LuminaColorScheme, LuminaColorSchemeManager } from '../theme';

interface Props {
	manager: LuminaColorSchemeManager;
	defaultColorScheme: LuminaColorScheme;
	forceColorScheme: LuminaColorScheme | undefined;
}

export function useProviderColorScheme({
	manager,
	defaultColorScheme,
	forceColorScheme,
}: Props) {
	const [value, setValue] = useState(() => manager.get(defaultColorScheme));
	const colorSchemeValue = forceColorScheme || value;

	const setColorScheme = useCallback(
		(colorScheme: LuminaColorScheme) => {
			if (!forceColorScheme) {
				setValue(colorScheme);
				manager.set(colorScheme);
			}
		},
		[manager, colorSchemeValue, forceColorScheme],
	);

	const clearColorScheme = useCallback(() => {
		setValue(defaultColorScheme);
		manager.clear();
	}, [manager, defaultColorScheme]);

	useEffect(() => {
		const unsubscribe = manager.subscribe(setColorScheme);
		return () => unsubscribe();
	}, [manager, setColorScheme]);

	return { colorScheme: colorSchemeValue, setColorScheme, clearColorScheme };
}

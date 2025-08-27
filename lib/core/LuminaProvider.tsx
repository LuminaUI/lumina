import React, {PropsWithChildren, useMemo} from '@rbxts/react';
import {createMemorySchemeManager} from './color-scheme-managers';
import {useProviderColorScheme} from '../hooks/use-provider-color-scheme';
import {usePx} from '../hooks/use-px';
import {LuminaContext} from './LuminaContext';
import {LuminaColorScheme, LuminaColorSchemeManager} from '../theme';

export interface LuminaProviderProps extends PropsWithChildren {
    colorSchemeManager?: LuminaColorSchemeManager;
    defaultColorScheme?: LuminaColorScheme;
    forceColorScheme?: LuminaColorScheme;
}

export function LuminaProvider({
                                   colorSchemeManager,
                                   defaultColorScheme = 'dark',
                                   forceColorScheme,
                                   children,
                               }: LuminaProviderProps) {
    usePx();
    const manager = useMemo(
        () => colorSchemeManager ?? createMemorySchemeManager(),
        [],
    );
	

    const {colorScheme, setColorScheme, clearColorScheme} =
        useProviderColorScheme({
            manager,
            defaultColorScheme,
            forceColorScheme,
        });

    return (
        <LuminaContext.Provider
            value={{colorScheme, setColorScheme, clearColorScheme}}
        >
            {children}
        </LuminaContext.Provider>
    );
}

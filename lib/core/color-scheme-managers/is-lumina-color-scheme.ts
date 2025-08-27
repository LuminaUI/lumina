import { LuminaColorScheme } from '../../theme';

export function isLuminaColorScheme(
	scheme: LuminaColorScheme,
): scheme is LuminaColorScheme {
	return scheme === 'dark' || scheme === 'light';
}

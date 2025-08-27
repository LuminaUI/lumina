import { isLuminaColorScheme } from './is-lumina-color-scheme';
import { LuminaColorScheme, LuminaColorSchemeManager } from '../../theme';

export function createMemorySchemeManager(): LuminaColorSchemeManager {
	// Use simple closure state instead of React hooks to avoid rules-of-hooks violations
	let currentScheme: LuminaColorScheme = 'dark';
	const subscribers = new Set<(value: LuminaColorScheme) => void>();

	let scheduled = false;

	const notify = () => {
		scheduled = false;
		for (const fn of subscribers) {
			fn(currentScheme);
		}
	};

	return {
		get: (defaultValue) =>
			isLuminaColorScheme(currentScheme) ? currentScheme : defaultValue,
		set: (value) => {
			if (value === currentScheme) return;

			currentScheme = value;

			if (!scheduled) {
				scheduled = true;
				task.defer(notify);
			}
		},
		subscribe(callback) {
			subscribers.add(callback);
			return () => subscribers.delete(callback);
		},
		clear: () => {
			currentScheme = 'dark';
		},
	};
}

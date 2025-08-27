import { useEffect } from '@rbxts/react';

export function useEvent<T extends Callback>(
	event: RBXScriptSignal<T>,
	callback: T,
) {
	useEffect(() => {
		const connection = event.Connect(callback);
		return () => connection.Disconnect();
	}, [event, callback]);
}

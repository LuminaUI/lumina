export function resolve_module_tree<T>([root, parts]: ReturnType<
	typeof $getModuleTree
>): T {
	let instance = root;

	for (const part of parts) {
		const child = instance.FindFirstChild(part);

		if (!child) {
			throw `Unable to find ${instance.GetFullName()}.${part}`;
		}

		instance = child;
	}

	return instance as T;
}

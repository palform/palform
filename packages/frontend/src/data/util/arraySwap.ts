export type ArrayMoveDirection = "up" | "down";

export function moveArrayItem<T extends { id: string }>(
	array: T[],
	item: T,
	direction: ArrayMoveDirection,
) {
	const itemIndex = array.findIndex((e) => e.id === item.id);

	if (
		(direction === "up" && itemIndex === 0) ||
		(direction === "down" && itemIndex === array.length - 1)
	)
		return;

	array.splice(itemIndex, 1);
	array.splice(direction === "up" ? itemIndex - 1 : itemIndex + 1, 0, item);
}

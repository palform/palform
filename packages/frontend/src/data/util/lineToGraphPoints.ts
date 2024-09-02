function yForX(intercept: number, gradient: number, x: number) {
    return gradient * x + intercept;
}

export function lineToGraphPoints(
    intercept: number,
    gradient: number,
    minX = -10,
    maxX = 10
): number[][] {
    const minY = yForX(intercept, gradient, minX);
    const maxY = yForX(intercept, gradient, maxX);

    return [
        [minX, minY],
        [maxX, maxY],
    ];
}

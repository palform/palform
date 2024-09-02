import Color from "colorjs.io";

export function colorWithLightness(
    color: string,
    lightness: number,
    alpha?: number
) {
    const col = new Color(color);
    col.hsl.l = lightness;
    if (alpha !== undefined) {
        col.alpha = alpha;
    }
    return col.toString({ format: "hex" });
}

import { faStar, faHeart, faThumbsUp } from "@fortawesome/free-solid-svg-icons";
import type { APIQuestionScaleIcon } from "@paltiverse/palform-typescript-openapi";

export function genScaleList(from: number, to: number) {
    const l: number[] = [];
    for (let i = from; i <= to; i++) {
        l.push(i);
    }
    return l;
}

export const scaleIcons: { name: string; value: APIQuestionScaleIcon }[] = [
    { name: "Numbers", value: "Numeric" },
    { name: "Stars", value: "Stars" },
    { name: "Hearts", value: "Hearts" },
    { name: "Thumbs up", value: "Thumbs" },
];

export function resolveScaleIcon(icon: APIQuestionScaleIcon) {
    switch (icon) {
        case "Numeric":
            return faStar;
        case "Stars":
            return faStar;
        case "Hearts":
            return faHeart;
        case "Thumbs":
            return faThumbsUp;
    }
}

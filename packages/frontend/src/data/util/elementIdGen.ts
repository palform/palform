export function generateFormListItemId(questionId: string, itemLabel: string) {
    return `${questionId}-${itemLabel.replaceAll(" ", "_")}`;
}

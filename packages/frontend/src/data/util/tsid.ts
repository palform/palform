export type TSIDResourceType = "question" | "question_group";
import { getTsid } from "tsid-ts";

export function randomIdForResource(resource: TSIDResourceType) {
    let prefix: string;

    switch (resource) {
        case "question":
            prefix = "qu";
            break;
        case "question_group":
            prefix = "qg";
            break;
        default:
            throw new Error(`unrecognised resource type ${resource}`);
    }

    const tsid = getTsid().toString();
    return `${prefix}_${tsid}`;
}

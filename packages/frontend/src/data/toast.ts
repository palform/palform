import { faCheck, faTimes } from "@fortawesome/free-solid-svg-icons";
import type { FontAwesomeIconProps } from "@fortawesome/svelte-fontawesome";
import { Mutex } from "async-mutex";
import { DateTime } from "luxon";
import { writable } from "svelte/store";
import { v4 as uuid } from "uuid";
import { humaniseAPIError } from "./common";

export interface ToastData {
    color?: "red" | "orange" | "green" | "blue";
    icon?: FontAwesomeIconProps["icon"];
    seconds?: number;
    label: string;
}
export type ToastDataInternal = ToastData & {
    id: string;
    seconds: number;
    created: DateTime;
    color: Required<ToastData["color"]>;
};
const toastsMutex = new Mutex();
export const toasts = writable<ToastDataInternal[]>([]);

export async function showToast(data: Omit<ToastData, "id">) {
    const id = uuid();

    const release = await toastsMutex.acquire();
    toasts.update((t) => {
        return [
            {
                ...data,
                id,
                seconds: data.seconds ?? 5,
                created: DateTime.now(),
                color: data.color ?? "blue",
            },
            ...t,
        ];
    });

    release();
    return id;
}

export function showSuccessToast(label: string) {
    return showToast({
        label,
        color: "green",
        icon: faCheck,
    });
}

export function showFailureToast(label: any) {
    return showToast({
        label: humaniseAPIError(label),
        color: "red",
        icon: faTimes,
    });
}

export async function removeToast(id: string) {
    const release = await toastsMutex.acquire();
    toasts.update((t) => {
        return t.filter((e) => e.id !== id);
    });
    release();
}

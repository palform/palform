<script lang="ts">
    import { removeToast, type ToastDataInternal } from "../../data/toast";
    import { slide } from "svelte/transition";
    import { onMount } from "svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faTimes } from "@fortawesome/free-solid-svg-icons";

    export let toast: ToastDataInternal;
    let toastOpen = true;
    const transitionDuration = 400;

    $: end = () => {
        removeToast(toast.id);
    };

    onMount(() => {
        const t = setTimeout(() => {
            toastOpen = false;

            setTimeout(() => {
                end();
            }, transitionDuration);
        }, toast.seconds * 1000);

        return () => clearTimeout(t);
    });
</script>

<div
    class={`border p-4 rounded-xl shadow flex items-center justify-between gap-4 ${toast.color === "red" ? "bg-red-600 border-red-500" : toast.color === "green" ? "bg-green-600 border-green-500" : toast.color === "orange" ? "bg-orange-600 border-orange-500" : toast.color === "blue" ? "bg-blue-600 border-blue-500" : ""}`}
    transition:slide={{ duration: transitionDuration }}
>
    <div class="flex items-center gap-4">
        {#if toast.icon}
            <div
                class="bg-white/70 h-8 w-8 flex items-center justify-center rounded-md"
            >
                <FontAwesomeIcon
                    icon={toast.icon}
                    class={toast.color === "red"
                        ? "text-red-600"
                        : toast.color === "green"
                          ? "text-green-600"
                          : toast.color === "orange"
                            ? "text-orange-600"
                            : toast.color === "blue"
                              ? "text-blue-600"
                              : ""}
                />
            </div>
        {/if}
        <p class="text-white">
            {toast.label}
        </p>
    </div>
    <div>
        <button class="ml-4 text-white" on:click={() => end()}>
            <FontAwesomeIcon icon={faTimes} />
        </button>
    </div>
</div>

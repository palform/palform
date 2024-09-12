<script lang="ts">
    import { onMount } from "svelte";
    import { getFormAdminContext } from "../../data/contexts/formAdmin";
    import {
        getFormEditorCtx,
        syncDirtyForm,
    } from "../../data/contexts/formEditor";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { get } from "svelte/store";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faCheckCircle,
        faTimesCircle,
    } from "@fortawesome/free-solid-svg-icons";
    import { showFailureToast } from "../../data/toast";

    const formEditorCtx = getFormEditorCtx();
    const formAdminCtx = getFormAdminContext();
    const orgCtx = getOrgContext();

    let syncing = false;
    let error = false;
    onMount(() => {
        let timeout: number | null = null;
        const unsub = formEditorCtx.subscribe(() => {
            if (timeout) {
                clearTimeout(timeout);
                timeout = null;
            }

            timeout = setTimeout(async () => {
                syncing = true;

                try {
                    await syncDirtyForm(
                        formEditorCtx,
                        formAdminCtx,
                        get(orgCtx).org.id
                    );
                    error = false;
                } catch (e) {
                    await showFailureToast(e);
                    error = true;
                }

                syncing = false;
                timeout = null;
            }, 200) as unknown as number;
        });

        return () => {
            unsub();
            if (timeout) {
                clearTimeout(timeout);
            }
        };
    });
</script>

<p
    class={`text-xs text-gray-500 dark:text-gray-400 ${syncing ? "animate-pulse" : ""}`}
>
    {#if syncing}
        Saving...
    {:else if error}
        <FontAwesomeIcon
            icon={faTimesCircle}
            class="text-red-400 dark:text-red-600"
        />
        Failed to save changes
    {:else}
        <FontAwesomeIcon
            icon={faCheckCircle}
            class="text-green-400 dark:text-green-600"
        />
        Changes saved
    {/if}
</p>

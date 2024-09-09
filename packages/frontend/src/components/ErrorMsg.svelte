<script lang="ts">
    import {
        faExclamationCircle,
        faRefresh,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Alert, Button } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";
    import { t } from "../data/contexts/i18n";

    export let e: any;
    export let targetDescriptor: string | undefined = undefined;
    export let retryable: boolean = false;

    const dispatch = createEventDispatcher<{ retry: undefined }>();
</script>

<Alert color="red" class={$$props.class}>
    <span slot="icon">
        <FontAwesomeIcon icon={faExclamationCircle} />
    </span>
    <p class="font-bold text-lg">
        {t("failed_to_load")}
        {targetDescriptor ?? ""}
    </p>
    <p>
        {e}
    </p>

    {#if retryable}
        <Button
            color="red"
            outline
            class="mt-2"
            on:click={() => dispatch("retry")}
        >
            <FontAwesomeIcon icon={faRefresh} class="me-2" />
            {t("try_again")}
        </Button>
    {/if}
</Alert>

<script lang="ts">
    import {
        faExclamationCircle,
        faRefresh,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Alert, Button } from "flowbite-svelte";
    import { t } from "../data/contexts/i18n";

    interface Props {
        e: any;
        targetDescriptor?: string;
        retryable?: boolean;
        class?: string;
        onretry?: () => void;
    }
    let {
        e,
        targetDescriptor = undefined,
        retryable = false,
        class: className = undefined,
        onretry,
    }: Props = $props();
</script>

<Alert color="red" class={className}>
    {#snippet icon()}
        <FontAwesomeIcon icon={faExclamationCircle} />
    {/snippet}
    <p class="font-bold text-lg">
        {t("failed_to_load")}
        {targetDescriptor ?? ""}
    </p>
    <p>
        {e}
    </p>

    {#if retryable}
        <Button color="red" outline class="mt-2" onclick={() => onretry?.()}>
            <FontAwesomeIcon icon={faRefresh} class="me-2" />
            {t("try_again")}
        </Button>
    {/if}
</Alert>

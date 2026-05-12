<script lang="ts">
    import { Alert, Button, P, Progressbar } from "flowbite-svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { rangeLerp } from "../../data/util/lerp";
    import { getFormAdminContext } from "../../data/contexts/formAdmin";

    interface Props {
        class?: string;
    }

    let { class: className }: Props = $props();

    const orgCtx = getOrgContext();
    const formAdminCtx = getFormAdminContext();
    let countLimit = $derived($orgCtx.entitlements?.question_per_form_count);
    let currentCount = $derived($formAdminCtx.questions.length);
    let progressValue = $derived(
        countLimit ? rangeLerp(0, countLimit, 0, 100, currentCount) : 0
    );
</script>

{#if countLimit}
    <Alert
        color={currentCount !== countLimit ? "secondary" : "primary"}
        border
        class={`${currentCount !== countLimit ? "border-gray-300 dark:border-gray-700" : ""} ${className}`}
    >
        <Progressbar class="mb-2" progress={progressValue} />
        {#if currentCount === countLimit}
            <P class="text-lg">
                You have reached your question limit ({countLimit}). Please
                upgrade your plan to add more.
            </P>
        {:else}
            <P>
                You have used {currentCount} out of {countLimit} questions included
                in your plan.
            </P>
        {/if}
        <Button class="mt-2" href={`/orgs/${$orgCtx.org.id}/settings/billing`}
            >Upgrade</Button
        >
    </Alert>
{/if}

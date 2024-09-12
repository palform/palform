<script lang="ts">
    import { Alert, Button, Progressbar } from "flowbite-svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { navigate } from "svelte-routing";
    import { rangeLerp } from "../../data/util/lerp";
    import { getFormAdminContext } from "../../data/contexts/formAdmin";

    const orgCtx = getOrgContext();
    const formAdminCtx = getFormAdminContext();
    $: countLimit = $orgCtx.entitlements?.question_per_form_count;
    $: currentCount = $formAdminCtx.questions.length;
    $: progressValue = countLimit
        ? rangeLerp(0, countLimit, 0, 100, currentCount)
        : 0;

    $: onUpgradeClick = () => {
        navigate(`/orgs/${$orgCtx.org.id}/settings/billing`);
    };
</script>

{#if countLimit}
    <Alert
        color={currentCount !== countLimit ? "light" : "primary"}
        border
        class={$$props.class}
    >
        <Progressbar class="mb-2" progress={progressValue} />
        {#if currentCount === countLimit}
            <p class="text-lg">
                You have reached your question limit ({countLimit}). Please
                upgrade your plan to add more.
            </p>
        {:else}
            <p>
                You have used {currentCount} out of {countLimit} questions included
                in your plan.
            </p>
        {/if}
        <Button class="mt-2" on:click={onUpgradeClick}>Upgrade</Button>
    </Alert>
{/if}

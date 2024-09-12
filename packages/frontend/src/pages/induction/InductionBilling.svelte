<script lang="ts">
    import { Button } from "flowbite-svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import InitiatePlan from "../../components/billing/InitiatePlan.svelte";
    import { navigate } from "svelte-routing";
    import BigAlert from "../../components/induction/bigAlert/BigAlert.svelte";
    import BigAlertText from "../../components/induction/bigAlert/BigAlertText.svelte";
    import BigAlertHeading from "../../components/induction/bigAlert/BigAlertHeading.svelte";

    const orgCtx = getOrgContext();
    let firstScreenComplete = false;

    const onFreePlanClick = () => {
        navigate(`/orgs/${$orgCtx.org.id}/induction/key`);
    };
</script>

{#if !firstScreenComplete}
    <BigAlert>
        <BigAlertHeading>Welcome to your new organisation!</BigAlertHeading>
        <BigAlertText class="mt-2">
            We're so happy to have you and your colleagues join Palform.
        </BigAlertText>
        <BigAlertText>
            Let's guide you through some simple steps to help you get the most
            out of your experience.
        </BigAlertText>

        <Button
            class="mt-4"
            size="lg"
            on:click={() => (firstScreenComplete = true)}
        >
            Get started
        </Button>
    </BigAlert>
{:else}
    <BigAlert class="mb-10">
        <BigAlertHeading>
            First, let's choose a plan that suits you
        </BigAlertHeading>
        <BigAlertText class="leading-tight mt-2">
            Our plans are designed to match your use case no matter the scale.
        </BigAlertText>
        <BigAlertText class="leading-tight">
            If there's some edge case we've missed, don't hesitate to get in
            touch!
        </BigAlertText>
        <BigAlertText class="mt-2">
            <strong>Can't decide now?</strong> Don't worry, you can stay on our free
            plan and upgrade at any time!
        </BigAlertText>

        <Button on:click={onFreePlanClick} class="mt-4">
            Continue on free plan
        </Button>
    </BigAlert>

    <InitiatePlan callbackPath="/induction/billing-complete" />
{/if}

<script lang="ts">
    import { type APIBillingSubscription } from "@paltiverse/palform-typescript-openapi";
    import CardBox from "../cardBox/CardBox.svelte";
    import CardBoxTitle from "../cardBox/CardBoxTitle.svelte";
    import CardBoxSubtitle from "../cardBox/CardBoxSubtitle.svelte";
    import { Button, Modal } from "flowbite-svelte";
    import CancelPlan from "./manage/CancelPlan.svelte";
    import { parseServerTime } from "../../data/util/time";
    import { DateTime } from "luxon";
    import UpcomingInvoicePreview from "./invoices/UpcomingInvoicePreview.svelte";
    import SwitchPlan from "./manage/SwitchPlan.svelte";
    import SectionSeparator from "../type/SectionSeparator.svelte";
    import { formatCurrency } from "@paltiverse/palform-frontend-common";

    export let subscription: APIBillingSubscription;

    let showManageModal = false;
    $: onCancel = () => {
        showManageModal = false;
        subscription.canceling_at_end = true;
    };
</script>

<CardBox>
    <CardBoxTitle>
        {subscription.plan_name}
        {#if subscription.is_trial}
            (14 day trial)
        {/if}
    </CardBoxTitle>
    <CardBoxSubtitle>
        {formatCurrency(
            subscription.currency,
            subscription.plan_frequency === "Monthly"
                ? subscription.price.amount
                : subscription.price.amount / 12
        )}/month
    </CardBoxSubtitle>
    {#if subscription.plan_frequency === "Annually"}
        <CardBoxSubtitle>
            Billed as {formatCurrency(
                subscription.currency,
                subscription.price.amount
            )}/year
        </CardBoxSubtitle>
    {/if}
    <p class="text-sm text-gray-500 dark:text-gray-400">
        Prices shown exclude tax (e.g. VAT)
    </p>
    {#if subscription.is_trial}
        <p class="text-sm text-gray-500 dark:text-gray-400">
            Once this trial ends, your payment method will automatically be
            charged the amount shown. You can cancel at any time before this.
        </p>
    {/if}

    {#if subscription.canceling_at_end}
        <CardBoxSubtitle class="mt-2 font-bold">
            Cancelling at end of period ({parseServerTime(
                subscription.period_end
            ).toLocaleString(DateTime.DATE_MED)})
        </CardBoxSubtitle>
    {/if}

    <UpcomingInvoicePreview {subscription} />

    <Button class="mt-2" on:click={() => (showManageModal = true)}>
        Manage
    </Button>
</CardBox>

<Modal
    bind:open={showManageModal}
    outsideclose
    title={`Manage ${subscription.plan_name}`}
    size="xl"
>
    <SwitchPlan {subscription} />

    <SectionSeparator />

    {#if subscription.canceling_at_end}
        <p>This plan will be cancelled at the end of its period.</p>
    {:else}
        <CancelPlan {subscription} on:cancel={onCancel} />
    {/if}
</Modal>

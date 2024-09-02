<script lang="ts">
    import type {
        APIBillingSubscription,
        APIBillingUpcomingInvoice,
    } from "@paltiverse/palform-typescript-openapi";
    import { Button, Modal, Spinner } from "flowbite-svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import UpcomingInvoiceTable from "../invoices/UpcomingInvoiceTable.svelte";
    import { formatCurrency } from "../../../data/billing/util";
    import { createEventDispatcher } from "svelte";
    import { parseServerTime } from "../../../data/util/time";
    import { DateTime } from "luxon";

    export let subscription: APIBillingSubscription;
    export let newPriceId: string;
    export let newPriceAmount: number;
    export let newPriceAnnual: boolean;
    export let open = false;

    const orgCtx = getOrgContext();
    const dispatch = createEventDispatcher<{ accept: undefined }>();

    let upcomingInvoice: APIBillingUpcomingInvoice | undefined;
    let initLoading = true;
    $: {
        initLoading = true;
        APIs.billingPlans()
            .then((a) =>
                a.billingPlanSwitch($orgCtx.org.id, true, {
                    new_stripe_price_id: newPriceId,
                })
            )
            .then((resp) => {
                upcomingInvoice = resp.data;
            })
            .catch(showFailureToast)
            .finally(() => (initLoading = false));
    }

    $: chargeTime = upcomingInvoice
        ? parseServerTime(upcomingInvoice.next_payment_attempt)
        : undefined;
    $: isChargeNow = chargeTime ? chargeTime <= DateTime.now() : false;
</script>

<Modal bind:open outsideclose title="Confirm new plan" size="lg">
    {#if initLoading}
        <Spinner size="14" />
    {:else if upcomingInvoice}
        <p>
            You will be charged the following invoice <strong>
                {isChargeNow
                    ? "now"
                    : "on " +
                      chargeTime?.toLocaleString(DateTime.DATE_MED)}</strong
            >.
        </p>

        <UpcomingInvoiceTable {upcomingInvoice} />

        <p>
            Then, the plan will continue at <strong
                >{formatCurrency(
                    subscription.currency,
                    newPriceAmount
                )}/{newPriceAnnual ? "year" : "month"}</strong
            > (+tax).
        </p>
        <p>
            You can canel or modify the plan whenever you want. By continuing,
            you agree to allow us to charge this amount to your payment method
            automatically.
        </p>

        <p>
            If you switch to a plan with fewer features than your current plan, <strong
                >those features and any resources created using them will become
                unavailable.</strong
            > Please make sure you have migrated away from such features before switching
            your plan.
        </p>

        <Button on:click={() => dispatch("accept")}>Great, continue!</Button>
    {/if}
</Modal>

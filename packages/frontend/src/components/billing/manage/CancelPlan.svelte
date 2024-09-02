<script lang="ts">
    import { Label, Select } from "flowbite-svelte";
    import SectionHeading from "../../type/SectionHeading.svelte";
    import type {
        APIBillingSubscription,
        CancelPlanRequestReason,
    } from "@paltiverse/palform-typescript-openapi";
    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { createEventDispatcher } from "svelte";
    import { parseServerTime } from "../../../data/util/time";
    import { DateTime } from "luxon";

    const orgCtx = getOrgContext();
    export let subscription: APIBillingSubscription;
    const dispatch = createEventDispatcher<{ cancel: undefined }>();

    const reasonItems: { name: string; value: CancelPlanRequestReason }[] = [
        { name: "Poor customer service", value: "CustomerService" },
        { name: "The quality of Palform is too low", value: "LowQuality" },
        { name: "I am missing some features", value: "MissingFeatures" },
        { name: "I am switching to another service", value: "SwitchedService" },
        {
            name: "Palform is too complicated for my needs",
            value: "TooComplex",
        },
        { name: "Palform costs too much", value: "TooExpensive" },
        { name: "I don't use Palform enough", value: "Unused" },
        { name: "Other/I don't want to specify", value: "Other" },
    ];

    let loading = false;
    let reason: CancelPlanRequestReason = "Other";

    $: onCancelClick = async () => {
        loading = true;

        try {
            await APIs.billingPlans().then((a) =>
                a.billingPlanCancel(
                    $orgCtx.org.id,
                    subscription.stripe_subscription_id,
                    {
                        reason,
                    }
                )
            );
            await showSuccessToast(
                "Plan cancelled. We're sorry to see you go :("
            );
            dispatch("cancel");
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    };
</script>

<SectionHeading>Cancel plan</SectionHeading>
<p>
    You're welcome to cancel your plan whenever you want. Please consider these
    points:
</p>

<ul class="list-disc list-inside">
    <li>You will be downgraded to our Free plan.</li>
    <li>
        Features in your plan will become available, which could mean <strong
            >some data will be lost</strong
        >. Please ensure you have migrated away from any features not available
        in the Free plan.
    </li>
    <li>
        You will not be charged any more for this subscription, except for any
        outstanding response overage fees.
    </li>
    {#if subscription.is_trial}
        <li>
            The cancellation will take effect <strong>immediately</strong> rather
            than at the end of the current trial period
        </li>
    {:else}
        <li>
            You'll be able to continue using your plan until the end of the
            current period (<strong
                >{parseServerTime(subscription.period_end).toLocaleString(
                    DateTime.DATE_MED
                )}</strong
            >).
        </li>
    {/if}
</ul>

<p>If you have any questions, please get in touch with us.</p>

<Label>
    Reason for cancellation
    <Select class="mt-2" items={reasonItems} bind:value={reason} />
</Label>

<LoadingButton
    outline
    color="red"
    {loading}
    disabled={loading}
    on:click={onCancelClick}
>
    Cancel {subscription.plan_name}
</LoadingButton>

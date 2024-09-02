<script lang="ts">
    import type {
        APIBillingCustomer,
        APIBillingSubscription,
    } from "@paltiverse/palform-typescript-openapi";
    import CardBox from "../cardBox/CardBox.svelte";
    import CardBoxTitle from "../cardBox/CardBoxTitle.svelte";
    import InfoText from "../type/InfoText.svelte";
    import AddressPreview from "./address/AddressPreview.svelte";
    import SectionHeading from "../type/SectionHeading.svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { APIs } from "../../data/common";
    import SkeletonPrimitive from "../SkeletonPrimitive.svelte";
    import InitiatePlan from "./InitiatePlan.svelte";
    import SectionSeparator from "../type/SectionSeparator.svelte";
    import ManageExistingPlan from "./ManageExistingPlan.svelte";
    import InvoiceList from "./invoices/InvoiceList.svelte";
    import CardBoxSubtitle from "../cardBox/CardBoxSubtitle.svelte";
    import LoadingButton from "../LoadingButton.svelte";
    import { showFailureToast } from "../../data/toast";

    export let customer: APIBillingCustomer;
    const orgCtx = getOrgContext();

    let existingSubscriptions: APIBillingSubscription[] | undefined = undefined;
    let existingSubscriptionsLoading = true;

    APIs.billingPlans()
        .then((a) => a.billingPlanGet($orgCtx.org.id))
        .then((resp) => {
            existingSubscriptions = resp.data;
            existingSubscriptionsLoading = false;
        });

    let paymentMethodLoading = false;
    $: onPaymentMethodUpdateClick = async () => {
        paymentMethodLoading = true;
        try {
            const linkResp = await APIs.billingCustomers().then((a) =>
                a.billingCustomerUpdatePaymentMethod($orgCtx.org.id, {
                    redirect_url: window.location.href,
                })
            );
            window.location.href = linkResp.data;
        } catch (e) {
            await showFailureToast(e);
        }
        paymentMethodLoading = false;
    };
</script>

{#if existingSubscriptionsLoading}
    <div class="space-y-2">
        <SkeletonPrimitive height="30px" />
        <SkeletonPrimitive height="420px" />
    </div>
{:else if existingSubscriptions === undefined || existingSubscriptions.length === 0}
    <SectionHeading class="mb-4">Get started</SectionHeading>
    <InitiatePlan />
{:else}
    <SectionHeading class="mb-2">
        Current plan{existingSubscriptions.length === 1 ? "" : "s"}
    </SectionHeading>
    <div class="space-y-4">
        {#each existingSubscriptions as subscription (subscription.stripe_subscription_id)}
            <ManageExistingPlan {subscription} />
        {/each}
    </div>
{/if}
<SectionSeparator />

<div class="grid lg:grid-cols-2 gap-10">
    <div>
        <SectionHeading>Billing contact</SectionHeading>
        <CardBox class="mt-2">
            {#if customer.entity_name}
                <CardBoxTitle>{customer.entity_name}</CardBoxTitle>
            {:else}
                <CardBoxTitle>No details registered yet</CardBoxTitle>
                <CardBoxSubtitle>
                    You'll provide your details when you sign up for a new plan.
                </CardBoxSubtitle>
            {/if}
            {#if customer.email}
                <CardBoxSubtitle>
                    {customer.email}
                </CardBoxSubtitle>
            {/if}
            {#if customer.address}
                <InfoText>
                    <AddressPreview address={customer.address} />
                </InfoText>
            {/if}

            {#if typeof customer.entity_name === "string"}
                <LoadingButton
                    buttonClass="mt-2"
                    size="sm"
                    outline
                    disabled={paymentMethodLoading}
                    loading={paymentMethodLoading}
                    on:click={onPaymentMethodUpdateClick}
                >
                    Update payment method
                </LoadingButton>
            {/if}
        </CardBox>
    </div>

    <div>
        <SectionHeading class="mb-2">Invoices</SectionHeading>
        <InvoiceList />
    </div>
</div>

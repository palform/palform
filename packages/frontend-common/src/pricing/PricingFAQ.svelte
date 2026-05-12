<script lang="ts">
    import { Accordion, AccordionItem } from "flowbite-svelte";
    import PricingFAQText from "./PricingFAQText.svelte";

    interface Props {
        class?: string;
    }

    let { class: className }: Props = $props();
</script>

<Accordion class={className}>
    <AccordionItem>
        {#snippet header()}
            <span>Is there a limit to responses?</span>
        {/snippet}
        <PricingFAQText>
            No! All our plans, including the free plan, come with unlimited
            responses.
        </PricingFAQText>
    </AccordionItem>
    <AccordionItem>
        {#snippet header()}
            <span>What payment methods do you accept?</span>
        {/snippet}
        <PricingFAQText>
            We use Stripe, so you can pay with any debit or credit card
            supported by its platform. If you need to pay with some other method
            (e.g. ACH/SEPA), please send us an email and we'll be happy to help!
        </PricingFAQText>
    </AccordionItem>
    <AccordionItem>
        {#snippet header()}
            <span>Is pricing per-user?</span>
        {/snippet}
        <PricingFAQText>
            No! One price includes all users. We think pricing per user would be
            silly, since an extra user basically adds zero extra cost for us.
            Processing responses and developing new features are the bulk of our
            expenses, so we prefer to make our pricing centered around those
            instead.
        </PricingFAQText>
    </AccordionItem>
    <AccordionItem>
        {#snippet header()}
            <span>What is a "team"?</span>
        {/snippet}
        <PricingFAQText>
            A logical collection of forms and associated branding schemes. Users
            in your organisation can be given access to multiple teams with a
            given permission level in each. This makes collaboration nice and
            transparent, even on a large scale.
        </PricingFAQText>
        <PricingFAQText>
            If you use OpenID Connect (on the Business plan), you can create
            rules to automatically assign users to teams based on a "groups"
            variable from your OIDC response.
        </PricingFAQText>
    </AccordionItem>
    <AccordionItem>
        {#snippet header()}
            <span>Do you offer discounts?</span>
        {/snippet}
        <PricingFAQText>Yes!</PricingFAQText>
        <PricingFAQText>
            <strong>Students</strong> at higher education institutions with ID
            can get our <strong>Pro</strong> plan for free.
        </PricingFAQText>
        <PricingFAQText>
            <strong>Non-profit organisations</strong> with government-issued
            proof of tax-free or charitable status can get our
            <strong>Pro</strong> plan for free.
        </PricingFAQText>
        <PricingFAQText>
            <strong>Educational institutions</strong> recognised by the relevant
            government agency, can get 25% off any plan.
        </PricingFAQText>
        <PricingFAQText>
            <strong>Startups</strong> can get our <strong>Pro</strong> plan for free,
            or 50% off any other plan, until 1 year from the date of founding.
        </PricingFAQText>
        <PricingFAQText>
            To claim any of these discounts, please send us an email. Discounts
            only apply to annual versions of plans; monthly plans cannot be
            discounted.
        </PricingFAQText>
    </AccordionItem>
    <AccordionItem>
        {#snippet header()}
            <span>Are there limits to file uploads?</span>
        {/snippet}
        <PricingFAQText>
            Our File Upload question currently only has limits on individual
            uploads: files larger than 10GB cannot be uploaded. We currently
            have no limit on overall files you can store, but we ask you to be
            considerate and delete any files you no longer need.
        </PricingFAQText>
        <PricingFAQText>
            We may introduce a limit if this starts costing us too much, but
            it's looking unlikely.
        </PricingFAQText>
        <PricingFAQText>
            There are no restrictions on types of files that can be uploaded.
        </PricingFAQText>
    </AccordionItem>
    <AccordionItem>
        {#snippet header()}
            <span>Are my responses secure?</span>
        {/snippet}
        <div class="space-y-2">
            <PricingFAQText>
                We pride ourselves on having best-in-class security: our
                mandatory end-to-end encryption provides far superior security
                compared to other commercial form builders, while minimising the
                impact to your experience.
            </PricingFAQText>
            <PricingFAQText>
                Don't rely on arbitrary promises: with Palform, it's <em
                    >mathematically impossible</em
                > for us (or any other intermediary) to see your responses.
            </PricingFAQText>
            <PricingFAQText>
                All data is stored in the European Union, simplifying compliance
                with the GDPR and reassuring your users.
            </PricingFAQText>
            <PricingFAQText>
                Everything, not just form responses, is encrypted using the
                industry-standard TLS protocol while in transit.
            </PricingFAQText>
            <PricingFAQText>
                Our server software and large components of our website are
                written in Rust, using popular and battle-tested open source
                software to ensure a durable experience.
            </PricingFAQText>
        </div>
    </AccordionItem>
    <AccordionItem>
        {#snippet header()}
            <span>Do your forms support mobile devices?</span>
        {/snippet}
        <PricingFAQText>
            Yes! We've designed our forms to work beautifully on all devices.
        </PricingFAQText>
    </AccordionItem>
    <AccordionItem>
        {#snippet header()}
            <span>How do I start?</span>
        {/snippet}
        <PricingFAQText>
            Simply choose a plan, create an account, and then answer a few easy
            questions. You'll be up and running in just 5-10 minutes.
        </PricingFAQText>
        <PricingFAQText>
            You'll be able to change your chosen plan before checkout.
        </PricingFAQText>
    </AccordionItem>
</Accordion>

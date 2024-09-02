<script>
    import { Accordion, AccordionItem } from "flowbite-svelte";
    import InfoText from "../../type/InfoText.svelte";
    import PlanFeatureItem from "./PlanFeatureItem.svelte";
</script>

<Accordion class={$$props.class}>
    <AccordionItem>
        <span slot="header">Free plan</span>
        <InfoText>
            If you don't subscribe to any plan, you're welcome to use Palform on
            our free plan without providing any payment details.
        </InfoText>
        <InfoText>This plan is limited to:</InfoText>
        <ul class="mt-2 space-y-2">
            <PlanFeatureItem feature="10 forms" />
            <PlanFeatureItem feature="20 questions/form" />
            <PlanFeatureItem feature="Unlimited responses" />
            <PlanFeatureItem feature="1 user" />
            <PlanFeatureItem feature="1 encryption key" />
            <PlanFeatureItem feature="Conditional section branching" />
            <PlanFeatureItem feature="All question types" />
            <PlanFeatureItem absence feature="Export your data" />
        </ul>
    </AccordionItem>
    <AccordionItem>
        <span slot="header">Is there a limit to responses?</span>
        No! All our plans, including the free plan, come with unlimited responses.
    </AccordionItem>
    <AccordionItem>
        <span slot="header">What payment methods do you accept?</span>
        We use Stripe, so you can pay with any debit or credit card supported by
        its platform, as well as a number of country-specific methods such as Bacs/SEPA
        Direct Debit, and ACH payments. Depending on your region, we also support
        Apple Pay and Google Pay.
    </AccordionItem>
    <AccordionItem>
        <span slot="header">Is pricing per-user?</span>
        No! One price includes all users. We think pricing per user would be silly,
        since an extra user basically adds zero extra cost for us. Processing responses
        and developing new features are the bulk of our expenses, so we prefer to
        make our pricing centered around those instead.
    </AccordionItem>
    <AccordionItem>
        <span slot="header">What is a "team"?</span>
        <p>
            A logical collection of forms and associated branding schemes. Users
            in your organisation can be given access to multiple teams with a
            given permission level in each. This makes collaboration nice and
            transparent, even on a large scale.
        </p>
        <p>
            If you use OpenID Connect (on the Business plan), you can create
            rules to automatically assign users to teams based on a "groups"
            variable from your OIDC response.
        </p>
    </AccordionItem>
    <AccordionItem>
        <span slot="header">Do you offer discounts?</span>
        <p>Yes!</p>
        <p>
            <strong>Students</strong> at higher education institutions with ID
            can get our <strong>Basic</strong> plan for free.
        </p>
        <p>
            <strong>Educational institutions</strong> recognised by the relevant
            government agency, can get 25% off any plan.
        </p>
        <p>
            <strong>Other non-profit organisations</strong> with government-issued
            proof of tax-free or charitable status can get 25% off any plan.
        </p>
        <p>
            <strong>Startups</strong> can get our <strong>Basic</strong> plan for
            free, or 50% off any other plan, until 1 year from the date of founding.
        </p>
        <p>
            To claim any of these discounts, please send us an email. Discounts
            only apply to annual versions of plans; monthly plans cannot be
            discounted.
        </p>
    </AccordionItem>
    <AccordionItem>
        <span slot="header">Can I increase the response limit?</span>
        <p>
            Yes: for a flat charge of <strong>£1.00/2000 responses</strong> (or €1.15)
            charged at the end of each billing cycle to your payment method.
        </p>
        <p>
            Partial charging of this fee is not allowed: for example, if your
            response limit is 5,000, you will be charged £1 on the 5,001st
            response.
        </p>
        <p>
            You can disable this charge in your dashboard, which will instead
            prevent new responses from being made.
        </p>
    </AccordionItem>
    <AccordionItem>
        <span slot="header">How do response limits work on annual plans?</span>
        <p>
            For annual plans, instead of a monthly limit, you'll get an annual
            limit equal to 12 times the normal monthly limit for the plan.
        </p>
        <p>
            For example, if you subscribe to an annual plan with a listed limit
            of 5,000 responses per month, your actual limit will be 60,000
            responses per year. You can distribute the responses throughout your
            billing year however you'd like.
        </p>
        <p>
            This means your response limit is clearly connected to a recurring
            payment and makes your invoices easier to understand.
        </p>
    </AccordionItem>
    <AccordionItem>
        <span slot="header">Can I use my promotion code?</span>
        <p>
            If you have a promotion code, you can apply it at checkout (after
            selecting a plan).
        </p>
    </AccordionItem>
    <AccordionItem>
        <span slot="header">Are my responses secure?</span>
        <div class="space-y-2">
            <p>
                We pride ourselves on having best-in-class security: our
                mandatory end-to-end encryption provides far superior security
                compared to other commercial form builders, while minimising the
                impact to your experience.
            </p>
            <p>
                Don't rely on arbitrary promises: with Palform, it's <em
                    >mathematically impossible</em
                > for us (or any other intermediary) to see your responses.
            </p>
            <p>
                All data is stored in the European Union, simplifying compliance
                with the GDPR and reassuring your users.
            </p>
            <p>
                Everything, not just form responses, is encrypted using the
                industry-standard TLS protocol while in transit.
            </p>
            <p>
                Our server software and large components of our website are
                written in Rust, using popular and battle-tested open source
                software to ensure a durable experience.
            </p>
        </div>
    </AccordionItem>
    <AccordionItem>
        <span slot="header">Do your forms support mobile devices?</span>
        <p>Yes! We've designed our forms to work beautifully on all devices.</p>
    </AccordionItem>
</Accordion>

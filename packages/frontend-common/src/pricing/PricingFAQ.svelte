<script>
    import { Accordion, AccordionItem } from "flowbite-svelte";
</script>

<Accordion class={$$props.class}>
    <AccordionItem>
        <span slot="header">Is there a limit to responses?</span>
        No! All our plans, including the free plan, come with unlimited responses.
    </AccordionItem>
    <AccordionItem>
        <span slot="header">What payment methods do you accept?</span>
        We use Stripe, so you can pay with any debit or credit card supported by
        its platform. If you need to pay with some other method (e.g. ACH/SEPA),
        please send us an email and we'll be happy to help!
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
            can get our <strong>Pro</strong> plan for free.
        </p>
        <p>
            <strong>Non-profit organisations</strong> with government-issued
            proof of tax-free or charitable status can get our
            <strong>Pro</strong> plan for free.
        </p>
        <p>
            <strong>Educational institutions</strong> recognised by the relevant
            government agency, can get 25% off any plan.
        </p>
        <p>
            <strong>Startups</strong> can get our <strong>Pro</strong> plan for free,
            or 50% off any other plan, until 1 year from the date of founding.
        </p>
        <p>
            To claim any of these discounts, please send us an email. Discounts
            only apply to annual versions of plans; monthly plans cannot be
            discounted.
        </p>
    </AccordionItem>
    <AccordionItem>
        <span slot="header">Are there limits to file uploads?</span>
        <p>
            Our File Upload question currently only has limits on individual
            uploads: files larger than 10GB cannot be uploaded. We currently
            have no limit on overall files you can store, but we ask you to be
            considerate and delete any files you no longer need.
        </p>
        <p>
            We may introduce a limit if this starts costing us too much, but
            it's looking unlikely.
        </p>
        <p>There are no restrictions on types of files that can be uploaded.</p>
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
    <AccordionItem>
        <span slot="header">How do I start?</span>
        <p>
            Simply choose a plan, create an account, and then answer a few easy
            questions. You'll be up and running in just 5-10 minutes.
        </p>
        <p>You'll be able to change your chosen plan before checkout.</p>
    </AccordionItem>
</Accordion>

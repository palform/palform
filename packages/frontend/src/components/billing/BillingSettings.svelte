<script lang="ts">
    import { Helper, Toggle } from "flowbite-svelte";
    import SectionHeading from "../type/SectionHeading.svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../data/toast";

    const orgCtx = getOrgContext();
    let loading = false;

    $: onOverageToggle = async (e: Event) => {
        const t = e.target as HTMLInputElement;

        loading = true;
        try {
            /*
            await APIs.billingSettings().then((a) =>
                a.billingSettingsOverage($orgCtx.org.id, {
                    allow_response_overage: t.checked,
                })
            );*/
            await showSuccessToast("Saved");
        } catch (e) {
            await showFailureToast(e);
            $orgCtx.org.billing_allow_overage = !t.checked;
        }

        loading = false;
    };
</script>

<SectionHeading>Settings</SectionHeading>

<Toggle
    class="mt-4"
    bind:checked={$orgCtx.org.billing_allow_overage}
    on:change={onOverageToggle}
    disabled={loading}
>
    <p>
        Allow response overage
        <Helper>
            Overage is charged at £1 (+tax) per 2000 responses or equivalent in
            your currency.
        </Helper>
        <Helper>
            If this is turned off, new responses beyond your limit will be
            blocked instead.
        </Helper>
        <Helper>
            This does not apply to the Free plan, which doesn't support overage.
        </Helper>
    </p>
</Toggle>

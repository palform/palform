<script lang="ts">
    import { Alert, Toggle } from "flowbite-svelte";
    import SectionHeading from "../../type/SectionHeading.svelte";
    import {
        getFormCtx,
        getOrgContext,
        updateFormCtx,
    } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import FormWebhooks from "./webhooks/FormWebhooks.svelte";

    const orgCtx = getOrgContext();
    const formMetadataCtx = getFormCtx();

    let email = $formMetadataCtx.notification_email;
    let saveLoading = false;
    $: onSave = async () => {
        saveLoading = true;
        try {
            await APIs.forms().then((a) =>
                a.formsUpdate($orgCtx.org.id, $formMetadataCtx.id, {
                    editor_name: $formMetadataCtx.editor_name,
                    title: $formMetadataCtx.title,
                    branding_id: $formMetadataCtx.branding_id,
                    notification_email: email,
                    end_configuration: $formMetadataCtx.end_configuration,
                    enable_captcha: $formMetadataCtx.enable_captcha,
                })
            );
            updateFormCtx(orgCtx, $formMetadataCtx.id, (f) => {
                f.notification_email = email;
            });
        } catch (e) {
            await showFailureToast(e);
        }
        saveLoading = false;
    };
</script>

<SectionHeading>Response notifications</SectionHeading>

<Toggle
    class="mt-4"
    bind:checked={email}
    disabled={saveLoading}
    on:change={onSave}
>
    Email
</Toggle>
{#if email}
    <Alert color="blue" class="mt-2">
        An email will be sent to all members of the team on each submission. The
        response itself will not be included in the email.
    </Alert>
{/if}

<FormWebhooks class="mt-6" />

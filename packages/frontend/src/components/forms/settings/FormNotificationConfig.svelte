<script lang="ts">
    import { Alert, Helper, Input, Label, Toggle } from "flowbite-svelte";
    import SectionHeading from "../../type/SectionHeading.svelte";
    import {
        getFormCtx,
        getOrgContext,
        updateFormCtx,
    } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import LoadingButton from "../../LoadingButton.svelte";

    const orgCtx = getOrgContext();
    const formCtx = getFormCtx();
    let settings = {
        email: $formCtx.notification_email,
        webhookURL: $formCtx.notification_webhook_url,
    };

    let enableWebhook = !!settings.webhookURL;
    const onWebhookToggleChange = (e: Event) => {
        const target = e.target as HTMLInputElement;
        if (!target.checked) {
            settings.webhookURL = null;
            return;
        }
        settings.webhookURL = "https://example.com";
    };

    $: changed =
        settings.email !== $formCtx.notification_email ||
        settings.webhookURL !== $formCtx.notification_webhook_url;

    let saveLoading = false;
    $: onSave = async () => {
        saveLoading = true;
        try {
            await APIs.forms().then((a) =>
                a.formsUpdate($orgCtx.org.id, $formCtx.id, {
                    editor_name: $formCtx.editor_name,
                    title: $formCtx.title,
                    branding_id: $formCtx.branding_id,
                    notification_email: settings.email,
                    notification_webhook_url: settings.webhookURL,
                    end_configuration: $formCtx.end_configuration,
                    enable_captcha: $formCtx.enable_captcha,
                })
            );
            updateFormCtx(orgCtx, $formCtx.id, (f) => {
                f.notification_email = settings.email;
                f.notification_webhook_url = settings.webhookURL;
            });
        } catch (e) {
            await showFailureToast(e);
        }
        saveLoading = false;
    };
</script>

<SectionHeading>Response notifications</SectionHeading>

<Toggle class="mt-4" bind:checked={settings.email} disabled={saveLoading}>
    Email
</Toggle>
{#if settings.email}
    <Alert color="blue" class="mt-2">
        An email will be sent to all members of the team on each submission. The
        response itself will not be included in the email.
    </Alert>
{/if}

<Toggle
    class="mt-4"
    bind:checked={enableWebhook}
    on:change={onWebhookToggleChange}
    disabled={saveLoading}
>
    Webhook
</Toggle>

{#if enableWebhook}
    <Label class="mt-4">
        Webhook URL
        <Input
            type="url"
            class="mt-1"
            bind:value={settings.webhookURL}
            disabled={saveLoading}
        />
        <Helper class="mt-2">
            A POST request will be sent to this URL on every submission
        </Helper>
    </Label>
{/if}

{#if changed}
    <LoadingButton
        buttonClass="mt-4"
        disabled={saveLoading}
        loading={saveLoading}
        on:click={onSave}>Save</LoadingButton
    >
{/if}

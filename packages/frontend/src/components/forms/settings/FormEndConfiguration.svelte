<script lang="ts">
    import { Input, Label, Toggle } from "flowbite-svelte";
    import {
        getFormCtx,
        getOrgContext,
        updateFormCtx,
    } from "../../../data/contexts/orgLayout";
    import SectionHeading from "../../type/SectionHeading.svelte";
    import MarkdownEditor from "../../markdown/MarkdownEditor.svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { APIs } from "../../../data/common";
    import type { APIForm } from "@paltiverse/palform-typescript-openapi";

    const orgCtx = getOrgContext();
    const formCtx = getFormCtx();

    let message = $formCtx.end_configuration.message ?? "";
    let showRestart = $formCtx.end_configuration.show_restart;
    let redirectURL = $formCtx.end_configuration.redirect_to;

    $: onRedirectToggleClick = (e: Event) => {
        const t = e.target as HTMLInputElement;
        if (t.checked) {
            redirectURL = "https://example.com";
        } else {
            redirectURL = null;
        }
    };

    $: hasChanged =
        message !== $formCtx.end_configuration.message ||
        showRestart !== $formCtx.end_configuration.show_restart ||
        redirectURL !== $formCtx.end_configuration.redirect_to;

    let loading = false;
    $: onSaveClick = async () => {
        loading = true;

        try {
            const updatedForm = {
                ...$formCtx,
                end_configuration: {
                    message,
                    show_restart: showRestart,
                    redirect_to: redirectURL,
                },
            } as APIForm;

            await APIs.forms().then((a) =>
                a.formsUpdate($orgCtx.org.id, $formCtx.id, updatedForm)
            );

            updateFormCtx(orgCtx, $formCtx.id, updatedForm);
            await showSuccessToast("End page configuration saved");
            loading = false;
        } catch (e) {
            await showFailureToast(e);
        }
    };
</script>

<SectionHeading>End page</SectionHeading>

<Label class="mt-4" for="message_editor">Message</Label>
<MarkdownEditor
    class="mt-1"
    bind:value={message}
    id="message_editor"
    disabled={loading}
/>

<Toggle class="mt-6" bind:checked={showRestart} disabled={loading}>
    Show button to start a new response
</Toggle>

<Toggle
    class="mt-4"
    checked={!!redirectURL}
    on:change={onRedirectToggleClick}
    disabled={loading}
>
    Redirect to a custom URL
</Toggle>

{#if !!redirectURL}
    <Label class="mt-4">
        URL
        <Input
            class="mt-1"
            type="url"
            bind:value={redirectURL}
            disabled={loading}
        />
    </Label>
{/if}

{#if hasChanged}
    <LoadingButton
        buttonClass="mt-4"
        on:click={onSaveClick}
        {loading}
        disabled={loading}>Save</LoadingButton
    >
{/if}

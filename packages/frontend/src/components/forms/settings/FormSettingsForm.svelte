<script lang="ts">
    import {
        Button,
        Helper,
        Input,
        Label,
        Select,
        Toggle,
        Tooltip,
    } from "flowbite-svelte";
    import { navigate, p } from "../../../router";
    import { APIs } from "../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import LoadingButton from "../../LoadingButton.svelte";
    import {
        getOrgContext,
        reloadInduction,
        updateFormCtx,
    } from "../../../data/contexts/orgLayout";
    import type {
        APIForm,
        APIFormBranding,
        UpdateFormRequest,
    } from "@palform/palform-typescript-openapi";
    import TeamDropdown from "../../teams/TeamDropdown.svelte";
    import { isEntitled } from "../../../data/billing/entitlement";

    interface Props {
        initialValue: (UpdateFormRequest & { id: string }) | undefined;
        initialTeamId?: string | undefined;
        oqpp?: boolean | undefined;
        selectField?: string | undefined;
    }

    let {
        initialValue,
        initialTeamId = undefined,
        oqpp = undefined,
        selectField = undefined,
    }: Props = $props();

    let isNew = $derived(initialValue === undefined);
    // svelte-ignore state_referenced_locally
    let editorName = $state(initialValue?.editor_name ?? "");
    // svelte-ignore state_referenced_locally
    let title = $state(initialValue?.title ?? "");
    // svelte-ignore state_referenced_locally
    let teamId = $state(initialTeamId ?? "");
    // svelte-ignore state_referenced_locally
    let captcha = $state(initialValue?.enable_captcha ?? false);
    let initialBrandingId = $derived(initialValue?.branding_id ?? "DEFAULT");
    // svelte-ignore state_referenced_locally
    let brandingId = $state(initialBrandingId);
    let loading = $state(false);

    let hasChanged = $derived(
        editorName !== initialValue?.editor_name ||
            title !== initialValue?.title ||
            captcha !== initialValue?.enable_captcha ||
            brandingId !== initialBrandingId
    );

    const ctx = getOrgContext();
    const isBrandingEntitled = isEntitled("branding_count");
    const isCaptchaEntitled = isEntitled("form_captcha");

    let brandings: APIFormBranding[] = $state([]);
    let brandingsLoading = $state(true);
    $effect(() => {
        if (teamId === "" || isNew) return;
        (async () => {
            brandingsLoading = true;
            const resp = await APIs.formBrandings().then((a) =>
                a.organisationTeamBrandingList($ctx.org.id, teamId)
            );
            brandings = resp.data;
            brandingsLoading = false;
        })();
    });

    let onSubmit = $derived(async (e: Event) => {
        e.preventDefault();
        if (!title || !brandingId || (!isNew && !editorName)) return;

        loading = true;
        const formsAPI = await APIs.forms();
        try {
            if (initialValue) {
                const updatedForm = {
                    ...initialValue,
                    editor_name: editorName,
                    title: title,
                    branding_id: brandingId === "DEFAULT" ? null : brandingId,
                    enable_captcha: captcha,
                } as APIForm;
                await formsAPI.formsUpdate(
                    $ctx.org.id,
                    initialValue.id,
                    updatedForm
                );
                updateFormCtx(ctx, initialValue.id, updatedForm);
            } else {
                if (oqpp === undefined) return;

                const resp = await formsAPI.formsCreate($ctx.org.id, {
                    editor_name: title,
                    title: title,
                    in_team: teamId,
                    one_question_per_page: oqpp,
                });
                await reloadInduction(ctx);
                ctx.update((ctx) => {
                    return {
                        ...ctx,
                        forms: [resp.data, ...ctx.forms],
                    };
                });
                navigate(`/orgs/${$ctx.org.id}/forms/${resp.data.id}/overview`);
            }
            await showSuccessToast(isNew ? "Form created" : "Form saved");
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    });
</script>

<form class="mt-4 space-y-4" onsubmit={onSubmit}>
    {#if isNew}
        <fieldset>
            <Label class="font-medium">
                Team to create in
                <TeamDropdown
                    class="mt-2"
                    bind:value={teamId}
                    disabled={loading}
                    required={true}
                    selectDefaultIfOnly
                />
            </Label>
        </fieldset>
    {/if}

    {#if !isNew}
        <fieldset>
            <Label class="font-medium">
                Form name
                <Input
                    required
                    bind:value={editorName}
                    disabled={loading}
                    class="mt-2"
                    autofocus={selectField === "editor_name" ? true : undefined}
                />
            </Label>
            <Helper class="mt-2">
                This is an internal name for the form, visible only to your
                organisation members.
            </Helper>
        </fieldset>
    {/if}
    {#if !oqpp || isNew}
        <fieldset>
            <Label class="font-medium">
                Form title
                <Input
                    required
                    bind:value={title}
                    disabled={loading}
                    class="mt-2"
                />
            </Label>
            <Helper class="mt-2">
                The public-facing title shown to anyone filling in your form.
            </Helper>
        </fieldset>
    {/if}

    {#if teamId !== "" && $isBrandingEntitled && !isNew}
        <fieldset>
            <Label class="font-medium">
                Branding scheme
                {#if brandingsLoading}
                    <Input value="Loading..." readonly class="mt-2" />
                {:else}
                    <Select
                        required
                        items={[
                            ...brandings.map((e) => ({
                                name: e.name,
                                value: e.id,
                            })),
                            { name: "Default", value: "DEFAULT" },
                        ]}
                        bind:value={brandingId}
                        class="mt-2"
                    />
                    <Helper class="mt-2">
                        You can <a
                            href={`/orgs/${$ctx.org.id}/settings/teams/${teamId}/brandings`}
                            class="hover:underline text-primary-800 dark:text-primary-400"
                        >
                            configure branding schemes
                        </a>
                        for this team.
                    </Helper>
                {/if}
            </Label>
        </fieldset>
    {/if}

    {#if !isNew}
        <Toggle
            bind:checked={captcha}
            disabled={loading || !$isCaptchaEntitled}
        >
            Protect responses with captcha
        </Toggle>
        {#if !$isCaptchaEntitled}
            <Tooltip placement="bottom-start"
                >Please upgrade your plan to enable this feature</Tooltip
            >
        {/if}
    {/if}

    {#if isNew || hasChanged}
        <LoadingButton disabled={loading} {loading} type="submit">
            {isNew ? "Create" : "Save"}
        </LoadingButton>
    {/if}
    {#if isNew}
        <Button
            href={p("/orgs/:orgId", { params: { orgId: $ctx.org.id } })}
            color="light"
        >
            Cancel
        </Button>
    {/if}
</form>

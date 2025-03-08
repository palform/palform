<script lang="ts">
    import {
        Button,
        ButtonGroup,
        Helper,
        Input,
        Label,
        Select,
        Toggle,
        Tooltip,
    } from "flowbite-svelte";
    import { Link, navigate } from "svelte-routing";
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
    } from "@paltiverse/palform-typescript-openapi";
    import TeamDropdown from "../../teams/TeamDropdown.svelte";
    import { isEntitled } from "../../../data/billing/entitlement";
    import { navigateEvent } from "@paltiverse/palform-frontend-common";

    export let initialValue: (UpdateFormRequest & { id: string }) | undefined;
    export let initialTeamId: string | undefined = undefined;
    export let oqpp: boolean | undefined = undefined;
    export let selectField: string | undefined = undefined;

    $: isNew = initialValue === undefined;
    let editorName = initialValue?.editor_name ?? "";
    let title = initialValue?.title ?? "";
    let teamId = initialTeamId ?? "";
    let captcha = initialValue?.enable_captcha ?? false;
    let loading = false;

    const ctx = getOrgContext();
    const isBrandingEntitled = isEntitled("branding_count");
    const isCaptchaEntitled = isEntitled("form_captcha");

    let brandings: APIFormBranding[] = [];
    let brandingsLoading = true;
    let brandingId = initialValue?.branding_id ?? "DEFAULT";
    $: (async () => {
        if (teamId === "" || isNew) return;
        brandingsLoading = true;
        const resp = await APIs.formBrandings().then((a) =>
            a.organisationTeamBrandingList($ctx.org.id, teamId)
        );
        brandings = resp.data;
        brandingsLoading = false;
    })();

    $: onSubmit = async (e: Event) => {
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
                navigate(`/orgs/${$ctx.org.id}/forms/${resp.data.id}/`);
            }
            await showSuccessToast(isNew ? "Form created" : "Form saved");
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    };
</script>

<form class="mt-4 space-y-4" on:submit={onSubmit}>
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
    {#if !oqpp}
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
                        You can <Link
                            to={`/orgs/${$ctx.org.id}/settings/teams/${teamId}/brandings`}
                            class="hover:underline text-primary-800 dark:text-primary-400"
                        >
                            configure branding schemes
                        </Link>
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

    <ButtonGroup>
        <LoadingButton
            disabled={loading}
            {loading}
            type="submit"
            outline={!isNew}
        >
            {isNew ? "Create!" : "Save!"}
        </LoadingButton>
        {#if isNew}
            <Button
                href={`/orgs/${$ctx.org.id}`}
                on:click={navigateEvent}
                outline
                color="primary"
            >
                Cancel
            </Button>
        {/if}
    </ButtonGroup>
</form>

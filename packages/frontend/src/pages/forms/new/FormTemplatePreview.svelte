<script lang="ts">
    import { type APIFormTemplate } from "@paltiverse/palform-typescript-openapi";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import {
        navigateEvent,
        TemplateFramePreview,
    } from "@paltiverse/palform-frontend-common";
    import TextButton from "../../../components/TextButton.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowLeft } from "@fortawesome/free-solid-svg-icons";
    import {
        getOrgContext,
        reloadInduction,
    } from "../../../data/contexts/orgLayout";
    import { Button, Modal } from "flowbite-svelte";
    import TeamDropdown from "../../../components/teams/TeamDropdown.svelte";
    import { navigate } from "svelte-routing";

    export let templateId: string;
    export let teamId: string | null = null;

    const orgCtx = getOrgContext();

    let template: APIFormTemplate | undefined = undefined;
    let templateLoading = true;
    APIs.formTemplates
        .formTemplatesGet(templateId)
        .then((resp) => {
            template = resp.data;
        })
        .catch(showFailureToast)
        .finally(() => (templateLoading = false));

    APIs.formTemplates
        .formTemplatesReportView(templateId)
        .catch(showFailureToast);

    let cloneLoading = false;
    let showTeamModal = false;
    let selectedTeam = teamId ?? "";
    $: onCloneClick = async () => {
        if ($orgCtx.myTeams.length > 1 && selectedTeam === "") {
            showTeamModal = true;
            return;
        }

        showTeamModal = false;
        cloneLoading = true;
        try {
            const newFormResp = await APIs.formTemplatesWithToken().then((a) =>
                a.formTemplatesClone($orgCtx.org.id, templateId, {
                    into_team: selectedTeam || $orgCtx.myTeams[0].team_id,
                })
            );

            await reloadInduction(orgCtx);
            orgCtx.update((ctx) => {
                return {
                    ...ctx,
                    forms: [newFormResp.data, ...ctx.forms],
                };
            });

            navigate(`/orgs/${$orgCtx.org.id}/forms/${newFormResp.data.id}/`);
        } catch (e) {
            await showFailureToast(e);
        }

        cloneLoading = false;
    };
</script>

<Modal
    bind:open={showTeamModal}
    title="Choose team to create form in"
    outsideclose
>
    <TeamDropdown bind:value={selectedTeam} />
    <svelte:fragment slot="footer">
        <Button on:click={onCloneClick}>Create</Button>
    </svelte:fragment>
</Modal>

<TextButton
    class="mb-4"
    href={`/orgs/${$orgCtx.org.id}/forms/templates/`}
    on:click={navigateEvent}
    disabled={cloneLoading}
>
    <FontAwesomeIcon icon={faArrowLeft} />
    Back to all templates
</TextButton>

{#if template}
    <TemplateFramePreview
        {template}
        appBaseURL={window.location.origin}
        showMarketing={false}
        buttonLinkToAuth={false}
        disabled={cloneLoading}
        on:click={onCloneClick}
    />
{/if}

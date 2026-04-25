<script lang="ts">
    import { type APIFormTemplate } from "@palform/palform-typescript-openapi";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import { TemplateFramePreview } from "@palform/palform-frontend-common";
    import TextButton from "../../../components/TextButton.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowLeft } from "@fortawesome/free-solid-svg-icons";
    import {
        getOrgContext,
        reloadInduction,
    } from "../../../data/contexts/orgLayout";
    import { Button, Modal } from "flowbite-svelte";
    import TeamDropdown from "../../../components/teams/TeamDropdown.svelte";
    import { navigate, p, route } from "../../../router";

    interface Props {
        templateId?: string | undefined;
        teamId?: string | null;
    }

    let { templateId, teamId = null }: Props = $props();

    const templateIdResolved = $derived(
        templateId ?? route.params.templateId ?? ""
    );

    const orgCtx = getOrgContext();

    let template: APIFormTemplate | undefined = $state(undefined);
    let templateLoading = $state(true);

    $effect(() => {
        const tid = templateIdResolved;
        if (!tid) return;

        templateLoading = true;
        APIs.formTemplates
            .formTemplatesGet(tid)
            .then((resp) => {
                template = resp.data;
            })
            .catch(showFailureToast)
            .finally(() => {
                templateLoading = false;
            });

        APIs.formTemplates.formTemplatesReportView(tid).catch(showFailureToast);
    });

    let cloneLoading = $state(false);
    let showTeamModal = $state(false);
    let selectedTeam = $state("");
    $effect(() => {
        selectedTeam = teamId ?? "";
    });
    let onCloneClick = $derived(async () => {
        if ($orgCtx.myTeams.length > 1 && selectedTeam === "") {
            showTeamModal = true;
            return;
        }

        showTeamModal = false;
        cloneLoading = true;
        try {
            const newFormResp = await APIs.formTemplatesWithToken().then((a) =>
                a.formTemplatesClone($orgCtx.org.id, templateIdResolved, {
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

            navigate(
                `/orgs/${$orgCtx.org.id}/forms/${newFormResp.data.id}/overview`
            );
        } catch (e) {
            await showFailureToast(e);
        }

        cloneLoading = false;
    });
</script>

<Modal
    bind:open={showTeamModal}
    title="Choose team to create form in"
    outsideclose
>
    <TeamDropdown bind:value={selectedTeam} />
    {#snippet footer()}
        <Button onclick={onCloneClick}>Create</Button>
    {/snippet}
</Modal>

<TextButton
    class="mb-4"
    href={p("/orgs/:orgId/forms/templates", {
        params: { orgId: $orgCtx.org.id },
    })}
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
        onclick={onCloneClick}
    />
{/if}

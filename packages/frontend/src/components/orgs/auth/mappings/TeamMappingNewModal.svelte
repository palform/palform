<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        Button,
        Helper,
        Input,
        Label,
        Modal,
        Select,
    } from "flowbite-svelte";
    import TeamDropdown from "../../../teams/TeamDropdown.svelte";
    import { orgMemberSelectItems } from "../../../../data/util/orgMemberEnum";
    import type {
        APIOrganisationAuthTeamMapping,
        APIOrganisationAuthTeamMappingRequest,
        OrganisationMemberRoleEnum,
    } from "@paltiverse/palform-typescript-openapi";
    import LoadingButton from "../../../LoadingButton.svelte";
    import { APIs } from "../../../../data/common";
    import { getOrgContext } from "../../../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../../../data/toast";
    import { createEventDispatcher } from "svelte";

    const orgCtx = getOrgContext();
    const dispatch = createEventDispatcher<{
        create: APIOrganisationAuthTeamMapping;
    }>();
    let showModal = false;
    let fieldValue = "";
    let teamId = "";
    let role: OrganisationMemberRoleEnum = "Viewer";
    let addLoading = false;

    $: onAdd = async (e: Event) => {
        e.preventDefault();
        addLoading = true;
        try {
            const mapping: APIOrganisationAuthTeamMappingRequest = {
                field_value: fieldValue,
                team_id: teamId,
                role,
            };

            const resp = await APIs.orgAuthTeamMappings().then((a) =>
                a.organisationAuthConfigMappingsCreate($orgCtx.org.id, mapping)
            );
            const team = await APIs.orgTeams().then((a) =>
                a.organisationTeamsGet($orgCtx.org.id, teamId)
            );

            dispatch("create", {
                ...mapping,
                team_name: team.data.name,
                id: resp.data,
            });
            await showSuccessToast("Team mapping created");
            showModal = false;
        } catch (e) {
            await showFailureToast(e);
        }

        addLoading = false;
    };
</script>

<Button class={$$props.class} on:click={() => (showModal = true)}>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Add rule
</Button>

<Modal bind:open={showModal} title="Add team mapping rule" outsideclose>
    <form class="space-y-6" on:submit={onAdd}>
        <Label>
            If OIDC groups field contains
            <Input
                bind:value={fieldValue}
                class="mt-2"
                required
                disabled={addLoading}
            />
            <Helper class="mt-2">
                You can change which field this refers to in your organisation's
                Authentication settings.
            </Helper>
        </Label>
        <Label>
            Then add to team
            <TeamDropdown
                bind:value={teamId}
                class="mt-2"
                required
                disabled={addLoading}
                allTeams
            />
        </Label>
        <Label>
            With role
            <Select
                items={orgMemberSelectItems()}
                class="mt-1"
                bind:value={role}
                disabled={addLoading}
            />
        </Label>

        <LoadingButton loading={addLoading} disabled={addLoading} type="submit">
            Add
        </LoadingButton>
    </form>
</Modal>

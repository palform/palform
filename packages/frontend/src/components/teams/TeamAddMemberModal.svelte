<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import type {
        APIOrgMember,
        APIOrganisationTeamMember,
        OrganisationMemberRoleEnum,
    } from "@paltiverse/palform-typescript-openapi";
    import {
        Button,
        Input,
        Label,
        Modal,
        MultiSelect,
        Select,
    } from "flowbite-svelte";
    import { APIs } from "../../data/common";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { orgMemberSelectItems } from "../../data/util/orgMemberEnum";
    import LoadingButton from "../LoadingButton.svelte";
    import { showFailureToast, showSuccessToast } from "../../data/toast";
    import { createEventDispatcher } from "svelte";

    export let teamId: string;
    export let existingTeamMemberIds: string[];
    const dispatch = createEventDispatcher<{
        add: APIOrganisationTeamMember;
    }>();
    const orgCtx = getOrgContext();
    let showModal = false;
    let members: APIOrgMember[] = [];
    let membersLoading = true;
    let selectedMemberIds: string[] = [];
    let role: OrganisationMemberRoleEnum = "Viewer";

    $: APIs.orgMembers()
        .then((a) => a.organisationMembersList($orgCtx.org.id))
        .then((resp) => {
            members = resp.data.filter(
                (e) => !existingTeamMemberIds.includes(e.user_id)
            );
            membersLoading = false;
        });

    let addLoading = false;
    $: onAddClick = async () => {
        addLoading = true;
        if (selectedMemberIds.length === 0) {
            return;
        }

        try {
            await APIs.orgTeamMembers().then((a) =>
                a.organisationTeamMembersAdd($orgCtx.org.id, teamId, {
                    user_ids: selectedMemberIds,
                    role,
                })
            );
            await showSuccessToast("Member added");

            for (const userId of selectedMemberIds) {
                const member = members.find((e) => e.user_id === userId)!;

                dispatch("add", {
                    user_id: userId,
                    user_email: member.user_email,
                    user_display_name: member.user_display_name,
                    role,
                });
            }
            showModal = false;
        } catch (e) {
            await showFailureToast(e);
        }

        addLoading = false;
    };
</script>

<Button on:click={() => (showModal = true)} class={$$props.class}>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Add member(s)
</Button>

<Modal
    bind:open={showModal}
    outsideclose
    title="Add members"
    classBody="!pb-40"
>
    <Label>
        Members
        {#if membersLoading}
            <Input readonly value="Loading..." class="mt-1" />
        {:else}
            <MultiSelect
                class="mt-1"
                disabled={addLoading}
                items={members.map((e) => ({
                    name:
                        e.user_email +
                        (e.user_display_name
                            ? ` (${e.user_display_name})`
                            : ""),
                    value: e.user_id,
                }))}
                bind:value={selectedMemberIds}
            />
        {/if}
    </Label>

    <Label>
        Role
        <Select
            items={orgMemberSelectItems()}
            class="mt-1"
            bind:value={role}
            disabled={addLoading}
        />
    </Label>

    <svelte:fragment slot="footer">
        <LoadingButton
            disabled={addLoading}
            loading={addLoading}
            on:click={onAddClick}
        >
            Add
        </LoadingButton>
    </svelte:fragment>
</Modal>

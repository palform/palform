<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import type {
        APIOrgMember,
        APIOrganisationTeamMember,
        OrganisationMemberRoleEnum,
    } from "@palform/palform-typescript-openapi";
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

    interface Props {
        teamId: string;
        existingTeamMemberIds: string[];
        class?: string;
        onadd: (newMember: APIOrganisationTeamMember) => void;
    }

    let {
        teamId,
        existingTeamMemberIds,
        onadd,
        class: className,
    }: Props = $props();
    const orgCtx = getOrgContext();
    let showModal = $state(false);
    let members: APIOrgMember[] = $state([]);
    let membersLoading = $state(true);
    let selectedMemberIds: string[] = $state([]);
    let role: OrganisationMemberRoleEnum = $state("Viewer");

    $effect(() => {
        APIs.orgMembers()
            .then((a) => a.organisationMembersList($orgCtx.org.id))
            .then((resp) => {
                members = resp.data.filter(
                    (e) => !existingTeamMemberIds.includes(e.user_id)
                );
                membersLoading = false;
            });
    });

    let addLoading = $state(false);
    let onAddClick = $derived(async () => {
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

                onadd({
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
    });
</script>

<Button onclick={() => (showModal = true)} class={className}>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Add member(s)
</Button>

<Modal
    bind:open={showModal}
    outsideclose
    title="Add members"
    bodyClass="pb-40!"
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

    {#snippet footer()}
        <LoadingButton
            disabled={addLoading}
            loading={addLoading}
            onclick={onAddClick}
        >
            Add
        </LoadingButton>
    {/snippet}
</Modal>

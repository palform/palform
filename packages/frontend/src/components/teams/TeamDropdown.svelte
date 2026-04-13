<script lang="ts">
    import { get } from "svelte/store";
    import { Select } from "flowbite-svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { APIs } from "../../data/common";
    import type { OrganisationMemberRoleEnum } from "@palform/palform-typescript-openapi";

    interface Props {
        value: string;
        disabled?: boolean;
        required?: boolean;
        allTeams?: boolean;
        selectDefaultIfOnly?: boolean;
        hideTeams?: string[];
        withRoleOnly?: OrganisationMemberRoleEnum | undefined;
        class?: string;
    }

    let {
        value = $bindable(),
        disabled = false,
        required = false,
        allTeams = false,
        selectDefaultIfOnly = false,
        hideTeams = [],
        withRoleOnly = undefined,
        class: className = "",
    }: Props = $props();

    const orgCtx = getOrgContext();

    let loading = $state(false);
    let items = $state<{ name: string; value: string }[]>(
        get(orgCtx).myTeams
            .filter(
                (e) =>
                    !hideTeams.includes(e.team_id) &&
                    (withRoleOnly === undefined || e.my_role === withRoleOnly),
            )
            .map((t) => ({
                name: t.name,
                value: t.team_id,
            })),
    );

    $effect(() => {
        if (!allTeams) {
            items = $orgCtx.myTeams
                .filter(
                    (e) =>
                        !hideTeams.includes(e.team_id) &&
                        (withRoleOnly === undefined || e.my_role === withRoleOnly),
                )
                .map((t) => ({
                    name: t.name,
                    value: t.team_id,
                }));
            loading = false;
            return;
        }

        loading = true;
        const orgId = $orgCtx.org.id;
        let cancelled = false;
        APIs.orgTeams()
            .then((a) => a.organisationTeamsList(orgId))
            .then((resp) => {
                if (!cancelled) {
                    items = resp.data.map((e) => ({
                        name: e.name,
                        value: e.id,
                    }));
                    loading = false;
                }
            });
        return () => {
            cancelled = true;
        };
    });

    $effect(() => {
        if (!selectDefaultIfOnly || loading || items.length !== 1) return;
        const only = items[0].value;
        if (value !== only) {
            value = only;
        }
    });
</script>

<Select
    class={className}
    {items}
    {required}
    disabled={disabled || loading}
    bind:value
/>

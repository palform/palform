<script lang="ts">
    import { Select } from "flowbite-svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { APIs } from "../../data/common";
    import type { OrganisationMemberRoleEnum } from "@paltiverse/palform-typescript-openapi";

    export let value: string;
    export let disabled = false;
    export let required = false;
    export let allTeams = false;
    export let selectDefaultIfOnly = false;
    export let hideTeams: string[] = [];
    export let withRoleOnly: OrganisationMemberRoleEnum | undefined = undefined;

    const orgCtx = getOrgContext();

    let items = $orgCtx.myTeams
        .filter(
            (e) =>
                !hideTeams.includes(e.team_id) &&
                (withRoleOnly === undefined || e.my_role === withRoleOnly)
        )
        .map((t) => ({
            name: t.name,
            value: t.team_id,
        }));

    let loading = false;
    $: {
        if (allTeams) {
            loading = true;
            APIs.orgTeams()
                .then((a) => a.organisationTeamsList($orgCtx.org.id))
                .then((resp) => {
                    items = resp.data.map((e) => ({
                        name: e.name,
                        value: e.id,
                    }));
                    loading = false;
                });
        }
    }

    $: {
        if (selectDefaultIfOnly && !loading && items.length === 1) {
            value = items[0].value;
        }
    }
</script>

<Select
    class={$$props.class ?? ""}
    {items}
    {required}
    disabled={disabled || loading}
    bind:value
/>

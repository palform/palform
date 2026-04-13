<script lang="ts">
    import type { APIForm } from "@palform/palform-typescript-openapi";
    import SidebarLink from "./SidebarLink.svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import type { MouseEventHandler } from "svelte/elements";

    interface Props {
        form: APIForm;
        onclick?: MouseEventHandler<HTMLAnchorElement>;
    }

    let { form, onclick }: Props = $props();
    const orgCtx = getOrgContext();

    let team = $derived(
        $orgCtx.myTeams.find((e) => e.team_id === form.team_id)
    );
</script>

<SidebarLink
    orgPath={`/forms/${form.id}/overview`}
    {onclick}
    activationLevel={2}
>
    {#if team !== undefined}
        <span class="block text-xs text-slate-600 dark:text-slate-400">
            {team.name}
        </span>
    {/if}
    {form.editor_name}
    <span
        class="px-2 py-1 bg-primary-200 text-primary-900 dark:bg-primary-800 dark:text-primary-100 rounded-full text-xs"
    >
        {form.response_count}
    </span>
</SidebarLink>

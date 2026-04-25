<script lang="ts">
    import FormSettingsForm from "../../components/forms/settings/FormSettingsForm.svelte";
    import MainTitle from "../../layouts/MainTitle.svelte";
    import FormOqppChoice from "../../components/forms/settings/FormOQPPChoice.svelte";
    import { isActive, route } from "../../router";

    interface Props {
        initialTeamId?: string | undefined;
        oqpp?: boolean | undefined;
    }

    let { initialTeamId = undefined, oqpp = $bindable(undefined) }: Props =
        $props();

    let initialTeamIdResolved = $derived.by(() => {
        if (initialTeamId !== undefined) {
            return initialTeamId;
        }

        if (isActive("/orgs/:orgId/forms/new/:initialTeamId")) {
            return route.getParams("/orgs/:orgId/forms/new/:initialTeamId")
                .initialTeamId;
        }

        return undefined;
    });
</script>

{#if oqpp === undefined}
    <MainTitle>Choose a form style</MainTitle>
    <FormOqppChoice class="mt-8" on:select={(e) => (oqpp = e.detail)} />
{:else}
    <MainTitle>Create a new form</MainTitle>
    <button
        class="text-left font-display dark:text-slate-300"
        onclick={() => (oqpp = undefined)}
    >
        {oqpp
            ? "with one question at a time"
            : "with multiple questions per page"}
    </button>
    <FormSettingsForm
        initialValue={undefined}
        initialTeamId={initialTeamIdResolved}
        {oqpp}
    />
{/if}

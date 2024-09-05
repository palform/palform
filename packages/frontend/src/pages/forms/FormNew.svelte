<script lang="ts">
    import FormSettingsForm from "../../components/forms/settings/FormSettingsForm.svelte";
    import MainTitle from "../../layouts/MainTitle.svelte";
    import FormOqppChoice from "../../components/forms/settings/FormOQPPChoice.svelte";

    export let initialTeamId: string | undefined = undefined;
    export let oqpp: boolean | undefined = undefined;
</script>

{#if oqpp === undefined}
    <MainTitle>Choose a form style</MainTitle>
    <FormOqppChoice class="mt-8" on:select={(e) => (oqpp = e.detail)} />
{:else}
    <MainTitle>Create a new form</MainTitle>
    <button
        class="text-left font-display dark:text-slate-300"
        on:click={() => (oqpp = undefined)}
    >
        {oqpp
            ? "with one question at a time"
            : "with multiple questions per page"}
    </button>
    <FormSettingsForm initialValue={undefined} {initialTeamId} {oqpp} />
{/if}

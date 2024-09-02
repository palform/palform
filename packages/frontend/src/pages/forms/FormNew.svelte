<script lang="ts">
    import FormSettingsForm from "../../components/forms/settings/FormSettingsForm.svelte";
    import MainTitle from "../../layouts/MainTitle.svelte";

    export let initialTeamId: string | undefined = undefined;
</script>

<MainTitle>Create a new form</MainTitle>
<FormSettingsForm initialValue={undefined} {initialTeamId} />

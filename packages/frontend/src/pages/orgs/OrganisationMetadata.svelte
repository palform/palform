<script lang="ts">
    import SectionHeading from "../../components/type/SectionHeading.svelte";
    import OrganisationNameForm from "../../components/orgs/settings/OrganisationNameForm.svelte";
    import DangerZone from "../../components/type/DangerZone.svelte";
    import SectionSeparator from "../../components/type/SectionSeparator.svelte";
    import OrganisationDelete from "../../components/orgs/settings/OrganisationDelete.svelte";
</script>

<SectionHeading>Rename your organisation</SectionHeading>
<OrganisationNameForm />

<SectionSeparator />
<DangerZone />

<OrganisationDelete class="mt-4" />

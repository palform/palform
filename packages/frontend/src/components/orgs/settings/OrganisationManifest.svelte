<script lang="ts">
    import type { APIOrganisationManifest } from "@palform/palform-typescript-openapi";
    import { Li, List, P } from "flowbite-svelte";

    interface Props {
        manifest: APIOrganisationManifest;
    }

    let { manifest }: Props = $props();
</script>

<P>
    <List>
        <Li>{manifest.form_count} form(s)</Li>
        <Li>{manifest.submission_count} submission(s)</Li>
        <Li>{manifest.question_count} question(s)</Li>
        <Li>{manifest.team_count} team(s)</Li>
        <Li>{manifest.branding_count} branding configuration(s)</Li>
        <Li>{manifest.member_count} member(s)</Li>
        <Li>{manifest.team_asset_count} assets(s)</Li>
        <Li>{manifest.audit_log_count} audit log entry(s)</Li>
    </List>
</P>

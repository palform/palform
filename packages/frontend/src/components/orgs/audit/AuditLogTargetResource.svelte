<script lang="ts">
    import type { APIAuditLogEntry } from "@palform/palform-typescript-openapi";
    import { Badge } from "flowbite-svelte";
    import TextButton from "../../TextButton.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowRight } from "@fortawesome/free-solid-svg-icons";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { p } from "../../../router";

    interface Props {
        entry: APIAuditLogEntry;
    }

    let { entry }: Props = $props();
    const orgCtx = getOrgContext();
    let linkTarget = $derived.by(() => {
        switch (entry.target_resource_type) {
            case "Branding":
                return p("/orgs/:orgId/settings/teams/:teamId/brandings", {
                    params: {
                        orgId: $orgCtx.org.id,
                        teamId: entry.target_resource_parent_ids[0],
                    },
                });
            case "Form":
                return p("/orgs/:orgId/forms/:formId/overview", {
                    params: {
                        orgId: $orgCtx.org.id,
                        formId: `form_${entry.target_resource_id}`,
                    },
                });
            case "OrganisationMember":
                return p("/orgs/:orgId/settings/members", {
                    params: {
                        orgId: $orgCtx.org.id,
                    },
                });
            case "Organisation":
                return p("/orgs/:orgId/settings/org", {
                    params: {
                        orgId: $orgCtx.org.id,
                    },
                });
            case "OrganisationAuthConfig":
                return p("/orgs/:orgId/settings/auth", {
                    params: {
                        orgId: $orgCtx.org.id,
                    },
                });
            case "OrganisationSubdomain":
                return p("/orgs/:orgId/settings/subdomain", {
                    params: {
                        orgId: $orgCtx.org.id,
                    },
                });
            case "Team":
                return p("/orgs/:orgId/settings/teams/:teamId/members", {
                    params: {
                        orgId: $orgCtx.org.id,
                        teamId: `team_${entry.target_resource_id}`,
                    },
                });
            case "TeamMember":
                return p("/orgs/:orgId/settings/teams/:teamId/members", {
                    params: {
                        orgId: $orgCtx.org.id,
                        teamId: entry.target_resource_parent_ids[0],
                    },
                });
        }
    });
    let targetResourceId = $derived.by(() => {
        let prefix = "";
        switch (entry.target_resource_type) {
            case "OrganisationMember":
            case "TeamMember":
                prefix = "user";
                break;
            case "Team":
                prefix = "team";
                break;
            case "Form":
                prefix = "form";
                break;
            case "Branding":
                prefix = "brand";
                break;
            case "OrganisationAuthConfig":
            case "OrganisationSubdomain":
            case "Organisation":
                prefix = "org";
                break;
            case "AuthSession":
                prefix = "au";
                break;
            case "Submission":
                prefix = "sub";
                break;
            case "AdminPublicKey":
                prefix = "admin_pk";
                break;
        }

        return `${prefix}_${entry.target_resource_id}`;
    });
</script>

<Badge>
    {entry.target_resource_type}
</Badge>
{#if linkTarget}
    <TextButton href={linkTarget}>
        View
        <FontAwesomeIcon icon={faArrowRight} />
    </TextButton>
{/if}
<span class="text-xs font-mono block mt-2">
    {targetResourceId}
</span>

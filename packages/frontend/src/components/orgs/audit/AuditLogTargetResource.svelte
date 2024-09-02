<script lang="ts">
    import type { AuditLogTargetResourceEnum } from "@paltiverse/palform-typescript-openapi";
    import { Badge } from "flowbite-svelte";
    import TextButton from "../../TextButton.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowRight } from "@fortawesome/free-solid-svg-icons";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { navigate } from "svelte-routing";

    export let id: string;
    export let resourceType: AuditLogTargetResourceEnum;
    const orgCtx = getOrgContext();
    $: linkTarget = () => {
        const path = `/orgs/${$orgCtx.org.id}`;
        switch (resourceType) {
            case "Branding":
                return `${path}/settings/teams`;
            case "Form":
                return `${path}/forms/${id}/`;
            case "OrganisationMember":
                return `${path}/settings/members`;
            case "Organisation":
                return `${path}/settings/org`;
            case "OrganisationAuthConfig":
                return `${path}/settings/auth`;
            case "OrganisationSubdomain":
                return `${path}/settings/subdomain`;
            case "Team":
                return `${path}/settings/teams`;
            case "TeamMember":
                return `${path}/settings/teams`;
        }
    };

    $: onViewClick = () => {
        const t = linkTarget();
        if (!t) return;
        navigate(t);
    };
</script>

<Badge>
    {resourceType}
</Badge>
{#if linkTarget()}
    <TextButton on:click={onViewClick}>
        View
        <FontAwesomeIcon icon={faArrowRight} />
    </TextButton>
{/if}
<span class="text-xs font-mono block mt-2">
    {id}
</span>

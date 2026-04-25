<script lang="ts">
    import { Button } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowRight } from "@fortawesome/free-solid-svg-icons";
    import type { APIOrganisation } from "@palform/palform-typescript-openapi";
    import CardBox from "../cardBox/CardBox.svelte";
    import CardBoxTitle from "../cardBox/CardBoxTitle.svelte";
    import { p } from "../../router";

    interface Props {
        org: APIOrganisation;
    }

    let { org }: Props = $props();
</script>

<CardBox>
    <CardBoxTitle class="mb-2">
        {org.display_name}
    </CardBoxTitle>
    <Button outline href={p(`/orgs/:orgId`, { params: { orgId: org.id } })}>
        Open <FontAwesomeIcon class="ml-2" icon={faArrowRight} />
    </Button>
</CardBox>

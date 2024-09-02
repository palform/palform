<script lang="ts">
    import { Button } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowRight } from "@fortawesome/free-solid-svg-icons";
    import { navigateEvent } from "../../utils/navigate";
    import type { APIOrganisation } from "@paltiverse/palform-typescript-openapi";
    import CardBox from "../cardBox/CardBox.svelte";
    import CardBoxTitle from "../cardBox/CardBoxTitle.svelte";

    export let org: APIOrganisation;
</script>

<CardBox>
    <CardBoxTitle class="mb-2">
        {org.display_name}
    </CardBoxTitle>
    <Button outline href={`/orgs/${org.id}`} on:click={navigateEvent}>
        Open <FontAwesomeIcon class="ml-2" icon={faArrowRight} />
    </Button>
</CardBox>

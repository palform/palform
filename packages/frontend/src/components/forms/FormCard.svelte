<script lang="ts">
    import type { APIForm } from "@paltiverse/palform-typescript-openapi";
    import { Button } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowRight } from "@fortawesome/free-solid-svg-icons";
    import { navigateEvent } from "../../utils/navigate";
    import CardBox from "../cardBox/CardBox.svelte";
    import CardBoxTitle from "../cardBox/CardBoxTitle.svelte";
    import CardBoxSubtitle from "../cardBox/CardBoxSubtitle.svelte";

    export let form: APIForm;
    export let orgId: string;
</script>

<CardBox>
    <CardBoxTitle>
        {form.editor_name}
    </CardBoxTitle>
    <CardBoxSubtitle class="mb-2">
        {form.response_count} response{form.response_count === 1 ? "" : "s"}
    </CardBoxSubtitle>
    <Button
        outline
        href={`/orgs/${orgId}/forms/${form.id}/`}
        on:click={navigateEvent}
    >
        View <FontAwesomeIcon class="ml-2" icon={faArrowRight} />
    </Button>
</CardBox>

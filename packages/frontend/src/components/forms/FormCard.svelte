<script lang="ts">
    import type { APIForm } from "@palform/palform-typescript-openapi";
    import { Button } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowRight } from "@fortawesome/free-solid-svg-icons";
    import CardBox from "../cardBox/CardBox.svelte";
    import CardBoxTitle from "../cardBox/CardBoxTitle.svelte";
    import CardBoxSubtitle from "../cardBox/CardBoxSubtitle.svelte";
    import { p } from "../../router";

    interface Props {
        form: APIForm;
        orgId: string;
    }

    let { form, orgId }: Props = $props();
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
        href={p("/orgs/:orgId/forms/:formId/overview", {
            params: { orgId, formId: form.id },
        })}
    >
        View <FontAwesomeIcon class="ml-2" icon={faArrowRight} />
    </Button>
</CardBox>

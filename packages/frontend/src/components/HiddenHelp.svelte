<script lang="ts">
    import {
        faArrowUpRightFromSquare,
        faQuestionCircle,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Accordion, AccordionItem, Button } from "flowbite-svelte";

    export let href: string;
</script>

<Accordion flush class={$$props.class}>
    <AccordionItem>
        <span slot="header" class="text-sm">
            <FontAwesomeIcon icon={faQuestionCircle} class="me-2" /> Help
        </span>

        <div class="space-y-2">
            <slot />
        </div>

        <Button size="xs" outline class="mt-4" {href} target="_blank">
            <FontAwesomeIcon icon={faArrowUpRightFromSquare} class="me-2" />
            More info
        </Button>
    </AccordionItem>
</Accordion>

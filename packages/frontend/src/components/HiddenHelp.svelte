<script lang="ts">
    import {
        faArrowUpRightFromSquare,
        faQuestionCircle,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Accordion, AccordionItem, Button } from "flowbite-svelte";

    interface Props {
        href: string;
        class?: string;
        children?: import("svelte").Snippet;
        [key: string]: any;
    }

    let { href, class: className, ...props }: Props = $props();
</script>

<Accordion flush class={className}>
    <AccordionItem>
        {#snippet header()}
            <span class="text-sm">
                <FontAwesomeIcon icon={faQuestionCircle} class="me-2" /> Help
            </span>
        {/snippet}

        <div class="space-y-2">
            {@render props.children?.()}
        </div>

        <Button size="xs" outline class="mt-4" {href} target="_blank">
            <FontAwesomeIcon icon={faArrowUpRightFromSquare} class="me-2" />
            More info
        </Button>
    </AccordionItem>
</Accordion>

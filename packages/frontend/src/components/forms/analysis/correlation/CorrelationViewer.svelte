<script lang="ts">
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import TextButton from "../../../TextButton.svelte";
    import { faMagicWandSparkles } from "@fortawesome/free-solid-svg-icons";
    import { Accordion, AccordionItem, Modal } from "flowbite-svelte";
    import CorrelationFromFeatureGroup from "./CorrelationFromFeatureGroup.svelte";
    import SectionSeparator from "../../../type/SectionSeparator.svelte";

    export let questionId: string;
    export let correlations: [string, [string, [string, number][]][]][];
    $: someCorrelations = correlations.some(([_, e]) =>
        e.some(([_, e]) => e.length > 0)
    );

    let showModal = false;
</script>

{#if someCorrelations}
    <TextButton class={$$props.class} on:click={() => (showModal = true)}>
        <FontAwesomeIcon icon={faMagicWandSparkles} class="me-2" />
        What influences these responses?
    </TextButton>
{/if}

<Modal bind:open={showModal} outsideclose title="Question correlation analysis">
    <p>Some weaker correlations may be hidden.</p>
    <SectionSeparator />

    {#each correlations as [fromFeatureLabel, targets] (fromFeatureLabel)}
        <CorrelationFromFeatureGroup
            fromQuestionId={questionId}
            {fromFeatureLabel}
            {targets}
        />
    {/each}

    <Accordion flush>
        <AccordionItem>
            <span slot="header" class="text-sm">What does this mean?</span>
            <p class="text-sm">
                Shown above are the <a
                    class="underline font-medium"
                    href="https://en.wikipedia.org/wiki/Pearson_correlation_coefficient"
                    target="_blank">Pearson correlation coefficients</a
                > for this question relative to other questions in the form. It measures
                the extent to which each pair of questions is correlated. A higher
                number indicates a stronger correlation, which could mean one variable
                influences another (although this is not always the case).
            </p>
        </AccordionItem>
    </Accordion>
</Modal>

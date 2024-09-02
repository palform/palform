<script lang="ts">
    import type { APIQuestionConfigurationOneOf3 } from "@paltiverse/palform-typescript-openapi";
    import {
        getEditorCtx,
        type QuestionEditEvents,
    } from "../../../data/contexts/questionsEditing";
    import { createEventDispatcher } from "svelte";
    import {
        Button,
        ButtonGroup,
        Input,
        Label,
        NumberInput,
        Select,
    } from "flowbite-svelte";
    import { genScaleList, scaleIcons } from "../../../data/util/scaleList";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faTrash } from "@fortawesome/free-solid-svg-icons";

    export let config: APIQuestionConfigurationOneOf3;
    const ctx = getEditorCtx();
    const dispatch = createEventDispatcher<QuestionEditEvents>();

    $: onUpdate = () => {
        dispatch("update", config);
    };

    $: toggleMinLabel = () => {
        config.scale.min_label =
            typeof config.scale.min_label === "string" ? null : "";
        dispatch("update", config);
    };
    $: toggleMaxLabel = () => {
        config.scale.max_label =
            typeof config.scale.max_label === "string" ? null : "";
        dispatch("update", config);
    };
</script>

<div
    class="w-full flex justify-between items-center mb-4 gap-2 overflow-x-auto"
>
    {#each genScaleList(config.scale.min, config.scale.max) as i}
        <p
            class="h-12 w-12 bg-primary-200 rounded-lg flex items-center justify-center text-xl text-primary-800"
        >
            {i}
        </p>
    {/each}
</div>
<div class="flex w-full justify-between">
    <div>
        <NumberInput
            placeholder="Min value"
            bind:value={config.scale.min}
            on:input={onUpdate}
            disabled={$ctx.loading}
        />
        {#if config.scale.icon === "Numeric"}
            {#if typeof config.scale.min_label === "string"}
                <ButtonGroup class="w-full mt-2">
                    <Input
                        placeholder="Label"
                        bind:value={config.scale.min_label}
                        on:input={onUpdate}
                        disabled={$ctx.loading}
                    />
                    <Button on:click={toggleMinLabel} disabled={$ctx.loading}>
                        <FontAwesomeIcon icon={faTrash} />
                    </Button>
                </ButtonGroup>
            {:else}
                <Button
                    color="light"
                    size="xs"
                    outline
                    class="mt-2"
                    on:click={toggleMinLabel}
                    disabled={$ctx.loading}
                >
                    Add label
                </Button>
            {/if}
        {/if}
    </div>
    <div class="flex flex-col items-end">
        <NumberInput
            placeholder="Max value"
            bind:value={config.scale.max}
            on:input={onUpdate}
            disabled={$ctx.loading}
        />
        {#if config.scale.icon === "Numeric"}
            {#if typeof config.scale.max_label === "string"}
                <ButtonGroup class="w-full mt-2">
                    <Input
                        placeholder="Label"
                        bind:value={config.scale.max_label}
                        on:input={onUpdate}
                        disabled={$ctx.loading}
                    />
                    <Button on:click={toggleMaxLabel} disabled={$ctx.loading}>
                        <FontAwesomeIcon icon={faTrash} />
                    </Button>
                </ButtonGroup>
            {:else}
                <Button
                    color="light"
                    size="xs"
                    outline
                    class="mt-2"
                    on:click={toggleMaxLabel}
                    disabled={$ctx.loading}
                >
                    Add label
                </Button>
            {/if}
        {/if}
    </div>
</div>

<Label class="mt-4">
    Icon
    <Select
        items={scaleIcons}
        class="mt-1"
        bind:value={config.scale.icon}
        on:change={onUpdate}
        disabled={$ctx.loading}
    />
</Label>

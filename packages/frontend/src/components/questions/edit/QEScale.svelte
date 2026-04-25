<script lang="ts">
    import type { ConfigScale } from "@palform/palform-typescript-openapi";
    import { Button, ButtonGroup, Input, Label, Select } from "flowbite-svelte";
    import { genScaleList, scaleIcons } from "../../../data/util/scaleList";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faTrash } from "@fortawesome/free-solid-svg-icons";
    import { getFormEditorCtx } from "../../../data/contexts/formEditor";

    interface Props {
        config: ConfigScale;
    }

    let { config = $bindable() }: Props = $props();
    const ctx = getFormEditorCtx();

    let toggleMinLabel = $derived(() => {
        config.scale.min_label =
            typeof config.scale.min_label === "string" ? null : "";
    });
    let toggleMaxLabel = $derived(() => {
        config.scale.max_label =
            typeof config.scale.max_label === "string" ? null : "";
    });
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
        <Input
            type="number"
            placeholder="Min value"
            bind:value={config.scale.min}
            disabled={$ctx.loading}
        />
        {#if config.scale.icon === "Numeric"}
            {#if typeof config.scale.min_label === "string"}
                <ButtonGroup class="w-full mt-2">
                    <Input
                        placeholder="Label"
                        bind:value={config.scale.min_label}
                        disabled={$ctx.loading}
                    />
                    <Button onclick={toggleMinLabel} disabled={$ctx.loading}>
                        <FontAwesomeIcon icon={faTrash} />
                    </Button>
                </ButtonGroup>
            {:else}
                <Button
                    color="light"
                    size="xs"
                    class="mt-2"
                    onclick={toggleMinLabel}
                    disabled={$ctx.loading}
                >
                    Add label
                </Button>
            {/if}
        {/if}
    </div>
    <div class="flex flex-col items-end">
        <Input
            type="number"
            placeholder="Max value"
            bind:value={config.scale.max}
            disabled={$ctx.loading}
        />
        {#if config.scale.icon === "Numeric"}
            {#if typeof config.scale.max_label === "string"}
                <ButtonGroup class="w-full mt-2">
                    <Input
                        placeholder="Label"
                        bind:value={config.scale.max_label}
                        disabled={$ctx.loading}
                    />
                    <Button onclick={toggleMaxLabel} disabled={$ctx.loading}>
                        <FontAwesomeIcon icon={faTrash} />
                    </Button>
                </ButtonGroup>
            {:else}
                <Button
                    size="xs"
                    color="light"
                    class="mt-2"
                    onclick={toggleMaxLabel}
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
        disabled={$ctx.loading}
    />
</Label>

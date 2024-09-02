<script lang="ts">
    import { ctxGetCurrentGroup } from "../../../data/contexts/fill";
    import BrandedSpan from "../../teams/brandings/BrandedSpan.svelte";
    import GenericBrandBox from "../../teams/brandings/GenericBrandBox.svelte";

    const currentGroup = ctxGetCurrentGroup();
</script>

{#if $currentGroup && ($currentGroup.title || $currentGroup.description)}
    <GenericBrandBox
        neutralBorder
        class={`bg-slate-50 dark:bg-slate-700 ${$$props.class}`}
    >
        {#if $currentGroup.title}
            <h2 class="dark:text-gray-200">
                <BrandedSpan ignoreColor sizeGroup="h1">
                    {$currentGroup.title}
                </BrandedSpan>
            </h2>
        {/if}
        {#if $currentGroup.description}
            <p class="text-gray-600 dark:text-gray-400">
                <BrandedSpan ignoreColor sizeGroup="p">
                    {$currentGroup.description}
                </BrandedSpan>
            </p>
        {/if}
    </GenericBrandBox>
{/if}

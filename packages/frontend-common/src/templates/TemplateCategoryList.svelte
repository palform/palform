<script lang="ts">
    import type { APIFormTemplateCategory } from "@paltiverse/palform-typescript-openapi";
    import TemplateCategoryListItem from "./TemplateCategoryListItem.svelte";

    export let categoryURL: (id: string) => string;
    export let categories: APIFormTemplateCategory[];
    export let selectedId: string | undefined = undefined;
    export let routed = false;
</script>

<ul class="p-4 bg-slate-100 dark:bg-slate-800 rounded-3xl w-full">
    <p class="text-sm text-gray-600 uppercase tracking-wider mb-2">
        Template categories
    </p>
    {#each categories as category (category.id)}
        <TemplateCategoryListItem
            {category}
            url={categoryURL(category.id)}
            selected={category.id === selectedId}
            {routed}
        />
    {/each}
</ul>

<script lang="ts">
    import type { APIFormTemplateCategory } from "@palform/palform-typescript-openapi";
    import TemplateCategoryListItem from "./TemplateCategoryListItem.svelte";

    interface Props {
        categoryURL: (id: string) => string;
        categories: APIFormTemplateCategory[];
        selectedId?: string | undefined;
    }

    let { categoryURL, categories, selectedId = undefined }: Props = $props();
</script>

<ul class="p-4 bg-slate-100 dark:bg-slate-800 rounded-3xl w-full">
    <p
        class="text-sm text-gray-600 dark:text-gray-400 uppercase tracking-wider mb-2"
    >
        Template categories
    </p>

    {#each categories as category (category.id)}
        <TemplateCategoryListItem
            {category}
            url={categoryURL(category.id)}
            selected={category.id === selectedId}
        />
    {/each}
</ul>

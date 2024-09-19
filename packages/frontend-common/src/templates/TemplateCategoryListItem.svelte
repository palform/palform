<script lang="ts">
    import type { APIFormTemplateCategory } from "@paltiverse/palform-typescript-openapi";
    import { Badge } from "flowbite-svelte";

    export let category: APIFormTemplateCategory;
    export let url: string;
    export let selected = false;
</script>

<li>
    <a
        class={`flex items-center justify-between gap-2 px-6 py-4 rounded-xl dark:text-gray-100 ${selected ? "bg-primary-100 dark:bg-primary-900" : "hover:bg-slate-200 dark:hover:bg-slate-700"}`}
        href={url}
    >
        <span>
            {category.name}
        </span>
        <Badge>
            {category.template_count}
        </Badge>
    </a>
</li>

<script lang="ts">
    import type { APIFormTemplateCategory } from "@palform/palform-typescript-openapi";
    import { Badge } from "flowbite-svelte";

    interface Props {
        category: APIFormTemplateCategory;
        url: string;
        selected?: boolean;
    }

    let { category, url, selected = false }: Props = $props();
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

<script lang="ts">
    import type { APIFormTemplate } from "@palform/palform-typescript-openapi";
    import { Card } from "flowbite-svelte";
    import TemplateItemStats from "./TemplateItemStats.svelte";

    interface Props {
        template: APIFormTemplate;
        link: string;
        class?: string;
    }

    let { template, link, class: className }: Props = $props();
</script>

<Card href={link} class={`w-full max-w-none p-4 ${className}`}>
    <h5
        class="text-gray-800 dark:text-gray-200 text-xl leading-tight font-medium"
    >
        {template.name}
    </h5>
    <p class="mt-2 leading-tight line-clamp-3 text-gray-600 dark:text-gray-400">
        {template.description}
    </p>

    <TemplateItemStats {template} class="mt-3" />
</Card>

<script lang="ts">
    import type { APIFormTemplate } from "@paltiverse/palform-typescript-openapi";
    import { Card } from "flowbite-svelte";
    import TemplateItemStats from "./TemplateItemStats.svelte";
    import { navigateEvent } from "../data/util/navigate";

    export let template: APIFormTemplate;
    export let link: string;
    export let routed = false;
</script>

<Card
    href={link}
    class="w-full max-w-none"
    on:click={routed ? navigateEvent : undefined}
>
    <h5
        class="text-gray-800 dark:text-gray-200 text-xl leading-tight font-medium"
    >
        {template.name}
    </h5>
    <p class="mt-2 leading-tight line-clamp-3">
        {template.description}
    </p>

    <TemplateItemStats {template} class="mt-3" />
</Card>

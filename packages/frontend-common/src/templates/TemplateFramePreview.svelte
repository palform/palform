<script lang="ts">
    import type { APIFormTemplate } from "@paltiverse/palform-typescript-openapi";
    import { Badge, Button } from "flowbite-svelte";
    import LittleIcons from "../decorations/LittleIcons.svelte";
    import TemplateItemStats from "./TemplateItemStats.svelte";

    export let categoryId: string | undefined = undefined;
    export let template: APIFormTemplate;
    export let appBaseURL: string;
    export let showMarketing = true;
    export let buttonLinkToAuth = true;
    export let disabled = false;

    const iframeURL = new URL(
        `/fill/${template.organisation_id}/${template.id}/?f=${template.preview_token}`,
        appBaseURL
    );

    const useTemplateURL = new URL(
        `/auth/signup/?intentTemplate=${template.id}`,
        appBaseURL
    );
</script>

<main class={$$props.class}>
    <h1 class="mb-2 text-3xl font-display dark:text-gray-100">
        {template.name}
    </h1>

    <TemplateItemStats
        {template}
        includeAuthor
        class="text-gray-600 dark:text-gray-400"
    />

    <div class="flex xl:flex-row flex-col gap-10 mt-8">
        <iframe
            src={iframeURL.toString()}
            class="xl:w-3/5 w-full h-[70vh] bg-slate-50/70 dark:bg-slate-800 border dark:border-gray-700 rounded-xl overflow-hidden"
            title="Embedded preview of the Palform template"
        ></iframe>

        <div class="flex-1">
            <p class="text-gray-700 dark:text-gray-200 text-lg">
                {template.description}
            </p>

            <Button
                class="mt-8"
                href={buttonLinkToAuth ? useTemplateURL.toString() : undefined}
                {disabled}
                on:click
            >
                Use this template
            </Button>

            {#if showMarketing}
                <LittleIcons class="mt-3" />

                {#if categoryId}
                    <Badge
                        class="mt-4"
                        href={`/templates/${categoryId}`}
                        {disabled}
                        border
                    >
                        See similar templates
                    </Badge>
                {/if}
                <Badge class="mt-4" href={`/`} border {disabled}>
                    Learn more about Palform
                </Badge>
            {/if}
        </div>
    </div>
</main>

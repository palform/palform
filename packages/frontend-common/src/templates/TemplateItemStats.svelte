<script lang="ts">
    import {
        faCodeFork,
        faEye,
        faUser,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import type { APIFormTemplate } from "@palform/palform-typescript-openapi";

    interface Props {
        template: APIFormTemplate;
        includeAuthor?: boolean;
        class?: string;
    }

    let { template, includeAuthor = false, class: className }: Props = $props();
</script>

<p class={`text-xs text-gray-700 dark:text-gray-300 ${className}`}>
    <span class="align-text-bottom">
        <FontAwesomeIcon icon={faEye} class="inline-block h-3" size="sm" />
        {template.views}
    </span>
    <span class="ms-2 align-text-bottom">
        <FontAwesomeIcon icon={faCodeFork} class="inline-block h-3" size="sm" />
        {template.clones}
    </span>
    {#if includeAuthor}
        <span class="ms-2 align-text-bottom">
            <FontAwesomeIcon icon={faUser} class="inline-block h-3" size="sm" />
            {template.author_name}
        </span>
    {/if}
</p>

<script lang="ts">
    import {
        faCodeFork,
        faEye,
        faUser,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import type { APIFormTemplate } from "@paltiverse/palform-typescript-openapi";

    export let template: APIFormTemplate;
    export let includeAuthor = false;
</script>

<p class={`text-xs ${$$props.class}`}>
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

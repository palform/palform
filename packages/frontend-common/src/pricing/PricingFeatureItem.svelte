<script lang="ts">
    import {
        faCheckCircle,
        faPlusCircle,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import type { Snippet } from "svelte";

    interface Props {
        plus?: boolean;
        children?: Snippet;
    }

    let { plus = false, children }: Props = $props();
</script>

<li
    class="flex items-center gap-4 text-gray-600 dark:text-gray-300 leading-tight"
>
    {#if plus}
        <FontAwesomeIcon
            icon={faPlusCircle}
            class="text-gray-500 dark:text-gray-400"
        />
    {:else}
        <FontAwesomeIcon
            icon={faCheckCircle}
            class="text-primary-600 dark:text-primary-400"
        />
    {/if}
    <span>
        {@render children?.()}
    </span>
</li>

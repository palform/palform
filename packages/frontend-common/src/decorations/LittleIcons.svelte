<script lang="ts">
    import { faCircleCheck } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";

    interface Props {
        class?: string;
        hideFree?: boolean;
    }

    let { class: className, hideFree = false }: Props = $props();
</script>

<ul
    class={`text-xs flex gap-x-4 gap-y-1 flex-wrap text-gray-600 dark:text-gray-400 ${className}`}
>
    <li class="block align-baseline">
        <FontAwesomeIcon icon={faCircleCheck} class="inline-block ms-2 h-3" />
        5-minute setup
    </li>
    {#if !hideFree}
        <li class="block align-baseline">
            <FontAwesomeIcon
                icon={faCircleCheck}
                class="inline-block ms-2 h-3"
            />
            Free forever
        </li>
    {/if}
    <li class="block align-baseline">
        <FontAwesomeIcon icon={faCircleCheck} class="inline-block ms-2 h-3" />
        No card required
    </li>
</ul>

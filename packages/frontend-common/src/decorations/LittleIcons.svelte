<script lang="ts">
    import { faCircleCheck } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";

    export let hideFree = false;
</script>

<ul
    class={`text-xs space-x-4 text-gray-600 dark:text-gray-400 ${$$props.class}`}
>
    <li class="inline-block align-baseline">
        <FontAwesomeIcon icon={faCircleCheck} class="inline-block ms-2 h-3" />
        5-minute setup
    </li>
    {#if !hideFree}
        <li class="inline-block align-baseline">
            <FontAwesomeIcon
                icon={faCircleCheck}
                class="inline-block ms-2 h-3"
            />
            Free forever
        </li>
    {/if}
    <li class="inline-block align-baseline">
        <FontAwesomeIcon
            icon={faCircleCheck}
            class="inline-block ms-2 h-3"
        />
        No card required
    </li>
</ul>

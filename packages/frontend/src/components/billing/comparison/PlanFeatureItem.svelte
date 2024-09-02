<script lang="ts">
    import {
        faCheckCircle,
        faPlusCircle,
        faTimesCircle,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";

    export let feature: string;
    export let plus = false;
    export let absence = false;
</script>

<li class="leading-tight text-gray-600 dark:text-gray-300 flex items-center">
    <FontAwesomeIcon
        icon={plus ? faPlusCircle : absence ? faTimesCircle : faCheckCircle}
        class={`${plus ? "text:gray-600 dark:text-gray-400" : absence ? "text-red-600 dark:text-red-400" : "text-primary-600 dark:text-primary-400"} me-3`}
    />
    <span>
        {feature}
    </span>
</li>

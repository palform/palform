<script>
    import { faCheckCircle } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
</script>

<li
    class="flex items-center gap-4 text-gray-600 dark:text-gray-300 leading-tight"
>
    <FontAwesomeIcon icon={faCheckCircle} class="text-primary-600" />
    <span>
        <slot />
    </span>
</li>

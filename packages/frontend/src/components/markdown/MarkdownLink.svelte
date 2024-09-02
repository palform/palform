<script lang="ts">
    import { faUpRightFromSquare } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";

    export let href: string;
</script>

<a {href} class="underline" target="_blank">
    <slot /><FontAwesomeIcon icon={faUpRightFromSquare} class="ms-2" />
</a>

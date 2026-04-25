<script lang="ts">
    import { faUpRightFromSquare } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import type { Snippet } from "svelte";

    interface Props {
        href: string;
        children?: Snippet;
    }

    let { href, children }: Props = $props();
</script>

<a {href} class="underline" target="_blank">
    {@render children?.()}<FontAwesomeIcon
        icon={faUpRightFromSquare}
        class="ms-2"
    />
</a>

<script lang="ts">
    import { faBars } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button, Dropdown } from "flowbite-svelte";
    interface Props {
        children?: import("svelte").Snippet;
    }

    let { children }: Props = $props();
</script>

<Button outline>
    <FontAwesomeIcon icon={faBars} />
</Button>
<Dropdown simple>
    {@render children?.()}
</Dropdown>

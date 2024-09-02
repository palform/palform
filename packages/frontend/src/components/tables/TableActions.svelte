<script lang="ts">
    import { faBars } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button, Dropdown } from "flowbite-svelte";
</script>

<Button outline>
    <FontAwesomeIcon icon={faBars} />
</Button>
<Dropdown>
    <slot />
</Dropdown>

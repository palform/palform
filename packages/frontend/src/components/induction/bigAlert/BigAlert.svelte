<script>
    import { Alert } from "flowbite-svelte";
</script>

<Alert color="primary" border class={`p-8 ${$$props.class}`}>
    <slot />
</Alert>

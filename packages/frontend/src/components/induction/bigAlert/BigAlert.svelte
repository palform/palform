<script lang="ts">
    import { Alert } from "flowbite-svelte";
    interface Props {
        children?: import('svelte').Snippet;
        [key: string]: any
    }

    let { ...props }: Props = $props();
</script>

<Alert color="primary" border class={`p-8 ${props.class}`}>
    {@render props.children?.()}
</Alert>

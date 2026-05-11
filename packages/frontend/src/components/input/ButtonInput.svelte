<script lang="ts">
    import { Button, Input } from "flowbite-svelte";
    import type { Snippet } from "svelte";

    interface Props {
        value?: string;
        disabled?: boolean;
        hideButton?: boolean;
        onclick?: () => void;
        children?: Snippet;
    }

    let {
        value = $bindable(),
        disabled,
        hideButton,
        onclick,
        children,
    }: Props = $props();
</script>

<div class="flex gap-x-2">
    <Input bind:value {disabled} />
    {#if !hideButton}
        <Button {disabled} {onclick}>
            {@render children?.()}
        </Button>
    {/if}
</div>

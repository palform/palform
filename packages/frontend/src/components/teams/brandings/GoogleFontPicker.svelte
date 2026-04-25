<script lang="ts">
    import { Input, Select } from "flowbite-svelte";
    import { APIs } from "../../../data/common";

    interface Props {
        selectedFont: string;
        disabled?: boolean;
        class?: string;
    }

    let {
        selectedFont = $bindable(),
        disabled = false,
        class: className,
    }: Props = $props();

    let fontNames: string[] = $state([]);
    let loading = $state(true);
    APIs.formBrandings()
        .then((a) => a.googleFonts())
        .then((resp) => {
            fontNames = resp.data as unknown as string[];
            loading = false;
        });
</script>

{#if loading}
    <Input value="Loading..." readonly class={className} />
{:else}
    <Select
        bind:value={selectedFont}
        items={fontNames.map((e) => ({ name: e, value: e }))}
        class={className}
        {disabled}
    />
{/if}

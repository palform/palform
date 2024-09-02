<script lang="ts">
    import { Input, Select } from "flowbite-svelte";
    import { APIs } from "../../../data/common";

    export let selectedFont: string;
    export let disabled = false;

    let fontNames: string[] = [];
    let loading = true;
    APIs.formBrandings()
        .then((a) => a.googleFonts())
        .then((resp) => {
            fontNames = resp.data;
            loading = false;
        });
</script>

{#if loading}
    <Input value="Loading..." readonly class={$$props.class} />
{:else}
    <Select
        bind:value={selectedFont}
        items={fontNames.map((e) => ({ name: e, value: e }))}
        class={$$props.class}
        {disabled}
    />
{/if}

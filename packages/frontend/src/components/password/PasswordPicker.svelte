<script lang="ts">
    import { Helper, Input, Progressbar } from "flowbite-svelte";
    import { zxcvbn, zxcvbnOptions } from "@zxcvbn-ts/core";
    import * as zxcvbnCommonPackage from "@zxcvbn-ts/language-common";
    import * as zxcvbnEnPackage from "@zxcvbn-ts/language-en";

    export let value: string;
    export let required = false;
    export let disabled = false;

    zxcvbnOptions.setOptions({
        translations: zxcvbnEnPackage.translations,
        graphs: zxcvbnCommonPackage.adjacencyGraphs,
        dictionary: {
            ...zxcvbnCommonPackage.dictionary,
            ...zxcvbnEnPackage.dictionary,
        },
    });

    $: strength = zxcvbn(value);
</script>

<Input bind:value class={$$props.class} {required} {disabled} type="password" />

{#if value.length > 0}
    <Progressbar
        class="mt-2"
        progress={strength.score * 25}
        color={strength.score === 4
            ? "green"
            : strength.score === 3
              ? "yellow"
              : "red"}
    />
    {#if strength.feedback.warning}
        <Helper class="mt-2 text-orange-700 dark:text-orange-300">
            {strength.feedback.warning}
        </Helper>
    {/if}
{/if}

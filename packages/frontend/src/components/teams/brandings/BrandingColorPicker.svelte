<script lang="ts">
    import { faDroplet } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Input, Label } from "flowbite-svelte";

    export let value: string;
    export let disabled = false;
    export let name: string;

    const presetColors = [
        "#3584e4",
        "#33d17a",
        "#f6d32d",
        "#ff7800",
        "#e01b24",
        "#9141ac",
        "#986a44",
        "#000000",
        "#ff0000",
    ];

    $: isPreset = value !== "#ff0000" && presetColors.includes(value);
    const onColorChange = (e: HTMLInputElement, color: string) => {
        if (e.checked) {
            value = color;
        }
    };
</script>

<div class="grid grid-cols-10 gap-2 mt-2">
    {#each presetColors as color, index (color)}
        <div>
            <input
                type="radio"
                {name}
                class="sr-only peer"
                id={color + "-" + name}
                checked={value === color ||
                    (index === presetColors.length - 1 && !isPreset)}
                on:change={(e) => onColorChange(e.currentTarget, color)}
                {disabled}
            />
            <label
                for={color + "-" + name}
                class="flex items-center justify-center h-8 w-full rounded-md cursor-pointer peer-checked:outline outline-slate-600"
                style:background-color={index === presetColors.length - 1
                    ? undefined
                    : color}
            >
                {#if index === presetColors.length - 1}
                    <FontAwesomeIcon icon={faDroplet} />
                {/if}
            </label>
        </div>
    {/each}
</div>

{#if !isPreset}
    <Label>
        Custom color
        <Input type="color" bind:value class="mt-2" {disabled} />
    </Label>
{/if}

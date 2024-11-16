<script lang="ts">
    import { faBan, faDroplet } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Input, Label } from "flowbite-svelte";

    export let value: string | null;
    export let disabled = false;
    export let name: string;
    export let includeNullOption = false;
    export let pastel = false;

    const _presetColors = pastel
        ? [
              "#aeccf2",
              "#a4f2c7",
              "#efe2a0",
              "#f7caa3",
              "#fcb0b4",
              "#e5a9f9",
              "#ad9e91",
              "#bababa",
              "#ff0000",
          ]
        : [
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

    $: isPreset =
        (value === null && includeNullOption) ||
        (value !== null && value !== "#ff0000" && presetColors.includes(value));

    $: presetColors = includeNullOption
        ? _presetColors.slice(1)
        : _presetColors;

    const onColorChange = (e: HTMLInputElement, color: string | null) => {
        if (e.checked) {
            value = color;
        }
    };
</script>

<div class="grid grid-cols-10 gap-2 mt-2">
    {#if includeNullOption}
        <div>
            <input
                type="radio"
                {name}
                class="sr-only peer"
                id={"NONE-" + name}
                checked={value === null}
                on:change={(e) => onColorChange(e.currentTarget, null)}
                {disabled}
            />
            <label
                for={"NONE-" + name}
                class="flex items-center justify-center h-8 w-full rounded-md cursor-pointer peer-checked:outline outline-slate-600"
            >
                <FontAwesomeIcon icon={faBan} class="text-red-600" />
            </label>
        </div>
    {/if}

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
    <Label class="mt-2">
        Custom color
        <Input type="color" bind:value class="mt-2" {disabled} />
    </Label>
{/if}

<script lang="ts">
    import { faBan, faDroplet } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Input, Label } from "flowbite-svelte";

    interface Props {
        value: string | undefined;
        disabled?: boolean;
        name: string;
        includeNullOption?: boolean;
        pastel?: boolean;
    }

    let {
        value = $bindable(),
        disabled = false,
        name,
        includeNullOption = false,
        pastel = false,
    }: Props = $props();

    const _presetColors = $derived(
        pastel
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
              ]
    );

    const onColorChange = (e: HTMLInputElement, color: string | undefined) => {
        if (e.checked) {
            value = color;
        }
    };
    let presetColors = $derived(
        includeNullOption ? _presetColors.slice(1) : _presetColors
    );
    let isPreset = $derived(
        (value === undefined && includeNullOption) ||
            (value !== undefined &&
                value !== "#ff0000" &&
                presetColors.includes(value))
    );
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
                onchange={(e) => onColorChange(e.currentTarget, undefined)}
                {disabled}
            />
            <label
                for={"NONE-" + name}
                class="flex items-center justify-center h-8 w-full rounded-md cursor-pointer peer-checked:outline-2 outline-slate-600"
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
                onchange={(e) => onColorChange(e.currentTarget, color)}
                {disabled}
            />
            <label
                for={color + "-" + name}
                class="flex items-center justify-center h-8 w-full rounded-md cursor-pointer peer-checked:outline-2 outline-slate-600"
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

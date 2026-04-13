<script lang="ts">
    import type { SizeProp } from "@fortawesome/fontawesome-svg-core";
    import type { IconDefinition } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button, ButtonGroup, Tooltip } from "flowbite-svelte";

    interface RadioButtonValue {
        label?: string;
        icon?: IconDefinition;
        iconSize?: SizeProp;
        value: string;
        tooltip?: string;
    }

    interface Props {
        selectedValue: string;
        values: RadioButtonValue[];
        conjoined?: boolean;
        class?: string;
    }

    let {
        selectedValue = $bindable(),
        values,
        conjoined = false,
        class: className,
    }: Props = $props();

    const groupId = $props.id();
    const buttonId = (value: string) => `tooltip-${groupId}-${value}`;

    $effect(() => {
        // See https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles/radiogroup_role#keyboard_interactions
        const evListener = (ev: KeyboardEvent) => {
            const focusedEl = document.activeElement;
            if (!focusedEl || !focusedEl.id.startsWith(`tooltip-${groupId}`))
                return;

            const selectedValueIndex = values.findIndex(
                (v) => focusedEl.id === buttonId(v.value)
            );

            if (selectedValueIndex === -1) return;

            let targetIndex = null;
            if (
                selectedValueIndex === 0 &&
                (ev.key === "ArrowLeft" || ev.key === "ArrowUp")
            ) {
                targetIndex = values.length - 1;
            } else if (
                selectedValueIndex === values.length - 1 &&
                (ev.key === "ArrowRight" || ev.key === "ArrowDown")
            ) {
                targetIndex = 0;
            } else if (ev.key === "ArrowLeft" || ev.key === "ArrowUp") {
                targetIndex = selectedValueIndex - 1;
            } else if (ev.key === "ArrowRight" || ev.key === "ArrowDown") {
                targetIndex = selectedValueIndex + 1;
            }

            if (targetIndex === null) return;

            const targetVal = values[targetIndex].value;
            const targetId = buttonId(targetVal);
            const targetEl = document.getElementById(targetId);
            if (targetEl === null) return;
            selectedValue = targetVal;
            targetEl.focus();
        };

        document.addEventListener("keyup", evListener);
        return () => document.removeEventListener("keyup", evListener);
    });
</script>

{#snippet buttonList()}
    {#each values as value (value.value)}
        {@const isSelected = selectedValue === value.value}
        <Button
            onclick={() => (selectedValue = value.value)}
            color={isSelected ? "primary" : "light"}
            id={buttonId(value.value)}
            role="radio"
            aria-checked={isSelected}
            tabindex={isSelected ? 0 : -1}
        >
            {#if value.icon !== undefined}
                <FontAwesomeIcon icon={value.icon} size={value.iconSize} />
            {/if}
            {#if value.label !== undefined}
                {value.label}
            {/if}
        </Button>
    {/each}
{/snippet}

{#if conjoined}
    <ButtonGroup class={className} role="radiogroup">
        {@render buttonList()}
    </ButtonGroup>
{:else}
    <div class={`flex gap-2 ${className}`} role="radiogroup">
        {@render buttonList()}
    </div>
{/if}

{#each values as value (value.value)}
    {#if value.tooltip !== undefined}
        <Tooltip triggeredBy={`#${buttonId(value.value)}`}>
            {value.tooltip}
        </Tooltip>
    {/if}
{/each}

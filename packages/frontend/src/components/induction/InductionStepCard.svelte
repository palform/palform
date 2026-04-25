<script lang="ts">
    import { Card } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faCheck } from "@fortawesome/free-solid-svg-icons";
    import type { Snippet } from "svelte";
    import type { IconProp } from "@fortawesome/fontawesome-svg-core";

    interface Props {
        title: string;
        href?: string | undefined;
        icon?: IconProp | undefined;
        checked?: boolean;
        disabled?: boolean;
        children?: Snippet;
        footer?: Snippet;
    }

    let {
        title,
        href = undefined,
        icon = undefined,
        checked = false,
        disabled = false,
        children,
        footer,
    }: Props = $props();
</script>

<Card
    class="max-w-full w-full p-4"
    href={checked || disabled ? undefined : href}
>
    {#if icon || checked}
        <p
            class={`text-xl ${!checked ? "text-white dark:text-gray-200 bg-primary-500 dark:bg-primary-700" : "text-green-100 dark:text-green-200 bg-green-400 dark:bg-green-800"} mb-2 flex rounded-full h-12 w-12 items-center justify-center`}
        >
            <FontAwesomeIcon icon={checked ? faCheck : (icon ?? faCheck)} />
        </p>
    {/if}

    <h2
        class={`tracking-tight font-semibold text-2xl font-display ${checked ? "line-through text-gray-500 dark:text-gray-500" : "text-gray-900 dark:text-gray-100"} ${disabled ? "text-gray-500! !dark:text-gray-500" : ""}`}
    >
        {title}
    </h2>
    <p class="mt-2 leading-tight text-gray-800 dark:text-gray-200">
        {@render children?.()}
    </p>

    <div>
        {@render footer?.()}
    </div>
</Card>

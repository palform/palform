<script lang="ts">
    import { Card } from "flowbite-svelte";
    import { navigateEvent } from "../../utils/navigate";
    import { type IconProp } from "@fortawesome/fontawesome-svg-core";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faCheck } from "@fortawesome/free-solid-svg-icons";

    export let title: string;
    export let href: string | undefined = undefined;
    export let icon: IconProp | undefined = undefined;
    export let checked = false;
    export let disabled = false;
</script>

<Card
    class="max-w-full w-full"
    href={checked || disabled ? undefined : href}
    on:click={checked || disabled ? undefined : navigateEvent}
>
    {#if icon || checked}
        <p
            class={`text-xl ${!checked ? "text-white dark:text-gray-200 bg-primary-500 dark:bg-primary-700" : "text-green-100 dark:text-green-200 bg-green-400 dark:bg-green-800"} mb-2 flex rounded-full h-12 w-12 items-center justify-center`}
        >
            <FontAwesomeIcon icon={checked ? faCheck : icon ?? faCheck} />
        </p>
    {/if}

    <h2
        class={`tracking-tight font-semibold text-2xl font-display ${checked ? "line-through text-gray-500 dark:text-gray-500" : "text-gray-900 dark:text-gray-100"} ${disabled ? "!text-gray-500 !dark:text-gray-500" : ""}`}
    >
        {title}
    </h2>
    <p class="mt-2 leading-tight">
        <slot />
    </p>

    <div>
        <slot name="footer" />
    </div>
</Card>

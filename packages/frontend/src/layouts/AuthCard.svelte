<script lang="ts">
    import { Card } from "flowbite-svelte";
    export let title: string | undefined = undefined;
</script>

<main class="flex justify-center items-center p-8 h-screen overflow-hidden">
    <div class="max-h-screen overflow-auto">
        {#if title}
            <h1
                class="dark:text-gray-100 text-3xl font-display mb-4 text-center"
            >
                {title}
            </h1>
        {/if}
        <Card class="xl:min-w-[600px] overflow-hidden">
            <slot />
        </Card>
        <div class="text-sm mt-4 text-center">
            <slot name="footer" />
        </div>
    </div>
</main>

<script lang="ts">
    import AuthCardHelp from "./AuthCardHelp.svelte";
    import textLogo from "../assets/text_logo.svg";

    export let title: string | undefined = undefined;
</script>

<main
    class="flex items-center h-screen overflow-hidden bg-slate-50/50 dark:bg-slate-900"
>
    <div
        class="h-screen flex flex-col justify-between py-12 overflow-y-auto overflow-x-hidden lg:w-1/2 w-full"
    >
        <a class="px-8 block" href="https://palform.app">
            <img src={textLogo} class="lg:h-12 h-8" alt="Palform logo" />
        </a>

        <div class="2xl:px-40 xl:px-30 lg:px-16 px-8">
            {#if title}
                <h1 class="dark:text-gray-200 text-4xl font-display mb-6">
                    {title}
                </h1>
            {/if}
            <div class="w-full">
                <slot />
            </div>
            <div class="text-sm mt-8">
                <slot name="footer" />
            </div>
        </div>

        <div class="lg:h-12" />
    </div>

    <div
        class="hidden lg:flex w-1/2 h-full justify-center items-center bg-gradient-to-tr from-primary-400 to-blue-400 dark:from-primary-800 dark:to-blue-800"
    >
        <AuthCardHelp />
    </div>
</main>

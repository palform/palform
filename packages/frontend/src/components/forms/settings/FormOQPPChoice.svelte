<script lang="ts">
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher<{ select: boolean }>();
</script>

<div class={`w-full grid lg:grid-cols-2 gap-8 ${$$props.class}`}>
    <button
        class="px-4 relative text-left border dark:border-slate-600 rounded-2xl flex items-center justify-center h-80 hover:bg-slate-100/80 dark:hover:bg-slate-800 shadow-sm"
        on:click={() => dispatch("select", true)}
    >
        <div class="max-w-64">
            <p class="dark:text-gray-400 font-display">What's your name?</p>
            <input
                class="h-7 border dark:border-slate-500 rounded-lg mt-1 dark:bg-slate-700 pointer-events-none"
            />
            <button
                class="mt-3 pointer-events-none bg-white dark:bg-slate-700 border dark:border-slate-500 rounded-lg px-2 py-1 text-sm dark:text-slate-400"
            >
                Next
            </button>
        </div>

        <p class="absolute bottom-3 left-4 dark:text-gray-300 text-lg">
            One question at a time
        </p>
    </button>

    <button
        class="px-4 relative text-left border dark:border-slate-600 rounded-2xl flex justify-center h-80 hover:bg-slate-100/80 dark:hover:bg-slate-800 shadow-sm"
        on:click={() => dispatch("select", false)}
    >
        <div class="max-w-64 mt-6">
            <p class="dark:text-gray-400 font-display">What's your name?</p>
            <input
                class="h-7 border dark:border-slate-500 rounded-lg mt-1 dark:bg-slate-700 pointer-events-none"
            />

            <p class="mt-4 dark:text-gray-400 font-display">How old are you?</p>
            <input
                class="h-7 border dark:border-slate-500 rounded-lg mt-1 dark:bg-slate-700 pointer-events-none"
            />

            <button
                class="mt-5 pointer-events-none bg-white dark:bg-slate-700 border dark:border-slate-500 rounded-lg px-2 py-1 text-sm dark:text-slate-400"
            >
                Next
            </button>
        </div>

        <p class="absolute bottom-3 left-4 dark:text-gray-300 text-lg">
            Multiple questions per page
        </p>
    </button>
</div>

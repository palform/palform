<script lang="ts">
    import NavSubItem from "./NavSubItem.svelte";

    export let href: string | undefined = undefined;
    export let subItems:
        | { title: string; description: string; href: string }[]
        | undefined = undefined;
    export let desktopOnly = false;
</script>

<li class={`group ${desktopOnly ? "hidden md:block" : ""}`}>
    <a {href} class="cursor-pointer">
        <slot />
    </a>

    {#if subItems !== undefined}
        <div
            class="fixed top-8 left-0 w-full p-8 opacity-0 pointer-events-none group-hover:opacity-100 group-hover:pointer-events-auto transition-opacity"
        >
            <div
                class="bg-slate-100/80 dark:bg-slate-800/80 backdrop-blur-lg shadow-lg shadow-slate-400/20 border border-slate-400/40 w-full rounded-2xl grid grid-cols-4 gap-4 p-4"
            >
                {#each subItems as item (item.title)}
                    <NavSubItem subItem={item} />
                {/each}
            </div>
        </div>
    {/if}
</li>

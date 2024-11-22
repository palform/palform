<script lang="ts">
    export let subItem: { title: string; description: string; href: string };
</script>

<a
    href={subItem.href}
    class="block px-4 py-3 rounded-lg hover:bg-slate-200/50 dark:hover:bg-slate-700 transition-colors"
>
    <span class="font-medium text-primary-800 dark:text-gray-200 block mb-2 leading-none"
        >{subItem.title}</span
    >
    <span
        class="block text-sm text-primary-800/70 dark:text-gray-400 font-normal font-sans leading-tight"
    >
        {subItem.description}
    </span>
</a>

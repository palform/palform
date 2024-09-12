<script lang="ts">
    import { getFormEditorCtx } from "../../../data/contexts/formEditor";

    export let title: string;
    export let description: string;

    const ctx = getFormEditorCtx();
</script>

<button
    class="bg-slate-50 hover:bg-slate-100 dark:bg-slate-700 dark:hover:bg-slate-600 rounded-md py-3 px-4 w-full block text-left"
    disabled={$ctx.loading}
    on:click
>
    <span class="text-slate-800 dark:text-slate-100 block">{title}</span>
    <span class="text-slate-700 dark:text-slate-300 text-sm block">
        {description}
    </span>
</button>

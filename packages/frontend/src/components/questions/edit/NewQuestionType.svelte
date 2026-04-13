<script lang="ts">
    import type { MouseEventHandler } from "svelte/elements";
    import { getFormEditorCtx } from "../../../data/contexts/formEditor";

    interface Props {
        title: string;
        description: string;
        onclick: MouseEventHandler<HTMLButtonElement>;
    }

    let { title, description, onclick }: Props = $props();

    const ctx = getFormEditorCtx();
</script>

<button
    class="bg-slate-50 hover:bg-slate-100 dark:bg-slate-700 dark:hover:bg-slate-600 rounded-md py-3 px-4 w-full block text-left"
    disabled={$ctx.loading}
    {onclick}
>
    <span class="text-slate-800 dark:text-slate-100 block">{title}</span>
    <span class="text-slate-700 dark:text-slate-300 text-sm block">
        {description}
    </span>
</button>

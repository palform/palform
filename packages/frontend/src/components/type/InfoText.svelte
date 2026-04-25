<script lang="ts">
    import type { Snippet } from "svelte";
    import type { HTMLAttributes } from "svelte/elements";

    interface Props extends HTMLAttributes<HTMLParagraphElement> {
        lighter?: boolean;
        class?: string;
        children?: Snippet;
    }

    let {
        lighter = false,
        class: className,
        children,
        ...restProps
    }: Props = $props();
</script>

<p
    class={`${lighter ? "text-slate-500 dark:text-slate-400" : "text-slate-900 dark:text-slate-300"} ${className ?? ""}`}
    {...restProps}
>
    {@render children?.()}
</p>

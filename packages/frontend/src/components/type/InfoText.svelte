<script lang="ts">
    export let lighter = false;
</script>

<p
    class={`${lighter ? "text-slate-500 dark:text-slate-400" : "text-slate-900 dark:text-slate-300"} ${$$props.class}`}
>
    <slot />
</p>

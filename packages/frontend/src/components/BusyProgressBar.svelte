<script lang="ts">
    import { onMount } from "svelte";

    let left = 0;
    onMount(() => {
        const interval = setInterval(() => {
            left++;

            if (left > 200) {
                left = -80;
            }
        }, 1);

        return () => clearInterval(interval);
    });
</script>

<div
    class="w-[180px] h-[6px] bg-gray-200 dark:bg-slate-800 relative overflow-hidden rounded-lg"
>
    <div
        class="absolute h-full w-[80px] rounded-lg bg-primary-600 dark:bg-primary-800"
        style:transform={`translateX(${left}px)`}
    />
</div>

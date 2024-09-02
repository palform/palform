<script lang="ts">
    import { toasts } from "../../data/toast";
    import ImperativeToast from "./ImperativeToast.svelte";
</script>

<div class="fixed bottom-0 right-0 m-4 space-y-4 z-50">
    {#each $toasts.toSorted( (a, b) => (a.created < b.created ? 1 : a.created === b.created ? 0 : -1) ) as toast (toast.id)}
        <ImperativeToast {toast} />
    {/each}
</div>

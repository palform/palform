<script lang="ts">
    import SkeletonPrimitive from "../SkeletonPrimitive.svelte";

    interface Props {
        class?: string;
    }
    let { class: className }: Props = $props();
</script>

<div class="space-y-2 {className}">
    <SkeletonPrimitive height="40px" />
    <SkeletonPrimitive height="60px" />
    <SkeletonPrimitive height="60px" />
    <SkeletonPrimitive height="60px" />
    <SkeletonPrimitive height="60px" />
    <SkeletonPrimitive height="60px" />
    <SkeletonPrimitive height="60px" />
</div>

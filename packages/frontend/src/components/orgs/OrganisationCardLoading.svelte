<script>
  import SkeletonPrimitive from "../SkeletonPrimitive.svelte";
</script>

<SkeletonPrimitive className="rounded-2xl p-4 space-y-4">
  <SkeletonPrimitive height="30px" className="rounded-lg" randomWidth />
  <SkeletonPrimitive height="20px" className="rounded-lg" randomWidth />
</SkeletonPrimitive>


<script lang="ts">
  export let className = "";
</script>

<h2 class={`text-xl ${className}`}>
  <slot />
</h2>

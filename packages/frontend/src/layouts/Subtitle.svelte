<script lang="ts">
  interface Props {
    className?: string;
    children?: import('svelte').Snippet;
  }

  let { className = "", children }: Props = $props();
</script>

<h2 class={`text-xl ${className}`}>
  {@render children?.()}
</h2>

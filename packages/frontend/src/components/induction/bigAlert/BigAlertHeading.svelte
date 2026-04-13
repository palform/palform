<script lang="ts">
	interface Props {
		children?: import('svelte').Snippet;
	}

	let { children }: Props = $props();
</script>

<h2 class="text-3xl font-bold tracking-tight">{@render children?.()}</h2>

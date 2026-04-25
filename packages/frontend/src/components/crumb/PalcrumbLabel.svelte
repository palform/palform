<script lang="ts">
  import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
  import type { PalcrumbPath } from "./types";

  interface Props {
    path: PalcrumbPath;
  }

  let { path }: Props = $props();
</script>

{#if path.icon !== undefined}
  <FontAwesomeIcon icon={path.icon} />
{/if}

{path.name}

<script lang="ts">
  import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
  import type { PalcrumbPath } from "./types";

  export let path: PalcrumbPath;
</script>

{#if path.icon !== undefined}
  <FontAwesomeIcon icon={path.icon} />
{/if}

{path.name}

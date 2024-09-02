<script lang="ts">
  import { Link } from "svelte-routing";
  import type { PalcrumbPath } from "./types";
  import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
  import { faChevronRight } from "@fortawesome/free-solid-svg-icons";
  import PalcrumbLabel from "./PalcrumbLabel.svelte";

  export let path: PalcrumbPath;
  export let isLast = false;
</script>

<li class={isLast ? "text-gray-600" : ""}>
  {#if path.href !== undefined}
    <Link to={path.href} class="hover:underline">
      <PalcrumbLabel {path} />
    </Link>
  {:else}
    <PalcrumbLabel {path} />
  {/if}
  {#if !isLast}
    <FontAwesomeIcon icon={faChevronRight} class="text-gray-400" />
  {/if}
</li>

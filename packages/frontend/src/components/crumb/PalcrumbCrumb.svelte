<script lang="ts">
  import type { PalcrumbPath } from "./types";
  import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
  import { faChevronRight } from "@fortawesome/free-solid-svg-icons";
  import PalcrumbLabel from "./PalcrumbLabel.svelte";

  interface Props {
    path: PalcrumbPath;
    isLast?: boolean;
  }

  let { path, isLast = false }: Props = $props();
</script>

<li class={isLast ? "text-gray-600" : ""}>
  {#if path.href !== undefined}
    <a href={path.href} class="hover:underline">
      <PalcrumbLabel {path} />
    </a>
  {:else}
    <PalcrumbLabel {path} />
  {/if}
  {#if !isLast}
    <FontAwesomeIcon icon={faChevronRight} class="text-gray-400" />
  {/if}
</li>

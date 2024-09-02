<script lang="ts">
  import { faHome } from "@fortawesome/free-solid-svg-icons";
  import PalcrumbCrumb from "./PalcrumbCrumb.svelte";
  import type { PalcrumbPath } from "./types";

  export let paths: PalcrumbPath[] = [];
  export let navClass = "";
</script>

<nav
  class={`px-4 py-2 bg-gray-100 border border-gray-200 shadow-sm shadow-gray-100 rounded-md ${navClass}`}
>
  <ol class="flex items-center space-x-4">
    <PalcrumbCrumb path={{ name: "Home", href: "/", icon: faHome }} />
    {#each paths as path, index}
      <PalcrumbCrumb {path} isLast={index === paths.length - 1} />
    {/each}
  </ol>
</nav>

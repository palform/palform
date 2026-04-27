<script lang="ts">
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { faArrowRight, faWarning } from "@fortawesome/free-solid-svg-icons";
    import { Button } from "flowbite-svelte";
    import { route } from "../../router";
    import { isOrgRouteMatch } from "../../data/routing";

    const ctx = getOrgContext();
    let shouldHide = $derived(
        $ctx.globalWarning === undefined
            ? false
            : $ctx.globalWarning.hideOnPaths.some((p) =>
                  isOrgRouteMatch(route.pathname, p, 1)
              )
    );
</script>

{#if $ctx.globalWarning && !shouldHide}
    <div class="bg-red-600 py-4 px-4 md:px-14 lg:px-32">
        <p class="text-white font-semibold flex items-center">
            <FontAwesomeIcon
                icon={faWarning}
                class="text-white me-4 text-2xl"
            />
            {$ctx.globalWarning.message}
        </p>
        <Button class="ms-10 mt-2" color="light" href={$ctx.globalWarning.link}>
            {$ctx.globalWarning.buttonText}
            <FontAwesomeIcon icon={faArrowRight} class="ms-2" />
        </Button>
    </div>
{/if}

<script lang="ts">
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { getBrandCtx } from "../../../data/contexts/brand";
    import { faUpRightFromSquare } from "@fortawesome/free-solid-svg-icons";

    export let dummyLinks = false;
    const brandCtx = getBrandCtx();
</script>

<footer class={$$props.class}>
    {#if $brandCtx !== undefined}
        <div>
            {#if $brandCtx.extra_footer_message}
                <p class="text-gray-600 text-sm">
                    {$brandCtx.extra_footer_message}
                </p>
            {/if}
            {#if $brandCtx.terms_link}
                <p class="text-gray-600 text-sm">
                    <a
                        href={$brandCtx.terms_link}
                        on:click={(e) => dummyLinks && e.preventDefault()}
                        target="_blank"
                        class="underline"
                    >
                        Terms
                        <FontAwesomeIcon icon={faUpRightFromSquare} />
                    </a>
                </p>
            {/if}
            {#if $brandCtx.privacy_link}
                <p class="text-gray-600 text-sm">
                    <a
                        href={$brandCtx.privacy_link}
                        on:click={(e) => dummyLinks && e.preventDefault()}
                        target="_blank"
                        class="underline"
                    >
                        Privacy Policy
                        <FontAwesomeIcon icon={faUpRightFromSquare} />
                    </a>
                </p>
            {/if}
        </div>
    {/if}
</footer>

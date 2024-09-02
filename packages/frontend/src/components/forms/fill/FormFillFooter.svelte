<script lang="ts">
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { getBrandCtx } from "../../../data/contexts/brand";
    import { faUpRightFromSquare } from "@fortawesome/free-solid-svg-icons";
    import { DateTime } from "luxon";

    export let dummyLinks = false;
    const brandCtx = getBrandCtx();
    const year = DateTime.now().year;
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

    {#if $brandCtx?.include_palform_attribution || $brandCtx === undefined}
        <div class="mt-4 text-gray-600 text-sm">
            <p>
                <a
                    href="https://palform.app"
                    on:click={(e) => dummyLinks && e.preventDefault()}
                    target="_blank"
                    class="hover:underline text-primary-700"
                >
                    Create secure forms for free with Palform
                    <FontAwesomeIcon icon={faUpRightFromSquare} />
                </a>
            </p>

            <p class="mt-1 text-xs">
                &copy; {year} Palform Ltd. registered company 15796859
            </p>
        </div>
    {/if}
</footer>

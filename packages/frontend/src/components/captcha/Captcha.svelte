<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";
    import { Turnstile } from "svelte-turnstile";

    const siteKey = import.meta.env.VITE_CAPTCHA_SITE_KEY;
    const dispatch = createEventDispatcher<{
        complete: string;
        clear: undefined;
    }>();

    const skipCaptcha = import.meta.env.VITE_SKIP_CAPTCHA === "true";

    onMount(() => {
        if (skipCaptcha) {
            dispatch("complete", "abc");
        }
    });
</script>

{#if !skipCaptcha}
    <Turnstile
        {siteKey}
        class={$$props.class}
        responseField={false}
        on:callback={(e) => dispatch("complete", e.detail.token)}
        on:timeout={() => dispatch("clear")}
        on:expired={() => dispatch("clear")}
        on:error={() => dispatch("clear")}
    />
{/if}

<script lang="ts">
    import { onMount } from "svelte";
    import { Turnstile } from "svelte-turnstile";

    interface Props {
        class?: string;
        oncomplete: (value: string) => void;
        onclear: () => void;
    }

    let { class: className, oncomplete, onclear }: Props = $props();

    const siteKey = import.meta.env.VITE_CAPTCHA_SITE_KEY;
    const skipCaptcha = import.meta.env.VITE_SKIP_CAPTCHA === "true";

    onMount(() => {
        if (skipCaptcha) {
            oncomplete("abc");
        }
    });
</script>

{#if !skipCaptcha}
    <Turnstile
        {siteKey}
        class={className}
        responseField={false}
        on:callback={(e) => oncomplete(e.detail.token)}
        on:timeout={() => onclear()}
        on:expired={() => onclear()}
        on:error={() => onclear()}
    />
{/if}

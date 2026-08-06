<script lang="ts">
    import { onMount } from "svelte";
    import "altcha";
    import type {} from "altcha/types/svelte";
    import { backendURL } from "../../data/common";
    import type { State } from "altcha/types";

    interface Props {
        class?: string;
        oncomplete: (value: string) => void;
        onclear: () => void;
    }

    let { class: className, oncomplete, onclear }: Props = $props();

    const skipCaptcha = import.meta.env.VITE_SKIP_CAPTCHA === "true";
    const captchaUrl = `${backendURL}/api/captcha`;

    onMount(() => {
        if (skipCaptcha) {
            oncomplete("abc");
        }
    });

    let onChange = (e: CustomEvent<{ payload?: string; state: State }>) => {
        const { payload } = e.detail;
        if (payload) {
            oncomplete(payload);
        } else {
            onclear();
        }
    };
</script>

{#if !skipCaptcha}
    <div class={className}>
        <altcha-widget
            auto="onfocus"
            challenge={captchaUrl}
            onstatechange={onChange}
        >
        </altcha-widget>
    </div>
{/if}

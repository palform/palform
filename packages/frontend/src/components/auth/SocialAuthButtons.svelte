<script lang="ts">
    import type { SocialAuthService } from "@palform/palform-typescript-openapi";
    import { APIs } from "../../data/common";
    import SocialAuthButton from "./SocialAuthButton.svelte";
    import { startSocialAuth } from "../../data/auth/social";
    import { showFailureToast } from "../../data/toast";

    interface Props {
        prefix?: string;
    }

    let { prefix = "" }: Props = $props();
    let providers: SocialAuthService[] | undefined = $state(undefined);
    APIs.auth
        .authSocialList()
        .then((resp) => (providers = resp.data.available_providers));

    let loading = $state(false);
    const onClick = async (service: SocialAuthService) => {
        loading = true;

        try {
            const url = await startSocialAuth(service);
            window.location.href = url;
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    };
</script>

{#if providers !== undefined}
    <div class="space-y-4">
        {#each providers as provider (provider)}
            <SocialAuthButton
                service={provider}
                {prefix}
                {loading}
                onclick={() => onClick(provider)}
            />
        {/each}
    </div>
{/if}

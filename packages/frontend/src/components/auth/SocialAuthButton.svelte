<script lang="ts">
    import type { SocialAuthService } from "@palform/palform-typescript-openapi";
    import LoadingButton from "../LoadingButton.svelte";
    import {
        socialAuthServiceIcon,
        socialAuthServiceName,
    } from "../../data/auth/social";

    interface Props {
        service: SocialAuthService;
        prefix?: string;
        loading?: boolean;
        onclick: () => void;
    }

    let { service, prefix = "", loading = false, onclick }: Props = $props();
</script>

<LoadingButton
    buttonClass="w-full"
    color="light"
    {onclick}
    {loading}
    disabled={loading}
>
    <img
        src={socialAuthServiceIcon(service)}
        class="w-5 me-4"
        role="presentation"
        alt={`{socialAuthServiceName(service)} logo`}
    />
    {prefix}
    {socialAuthServiceName(service)}
</LoadingButton>

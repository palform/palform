<script lang="ts">
    import type { SocialAuthService } from "@paltiverse/palform-typescript-openapi";
    import LoadingButton from "../LoadingButton.svelte";
    import {
        socialAuthServiceIcon,
        socialAuthServiceName,
    } from "../../data/auth/social";

    export let service: SocialAuthService;
    export let prefix = "";
    export let loading = false;
</script>

<LoadingButton
    buttonClass="w-full"
    color="light"
    on:click
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

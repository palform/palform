<script lang="ts">
    import type { APIAdminUserSecondAuthenticationFactor } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher } from "svelte";
    import WebauthnButton from "../../../auth/WebauthnButton.svelte";
    import { DateTime } from "luxon";

    export let nickname: string;
    const dispatch = createEventDispatcher<{
        enroll: APIAdminUserSecondAuthenticationFactor;
    }>();

    $: onEnroll = (e: CustomEvent<string>) => {
        dispatch("enroll", {
            id: e.detail,
            nickname,
            created_at: DateTime.now().toISO(),
        });
    };
</script>

<p>To continue, please click the button.</p>

<WebauthnButton
    flowType="register"
    registerNickname={nickname}
    on:enroll={onEnroll}
/>

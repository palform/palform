<script lang="ts">
    import type { APIAdminUserSecondAuthenticationFactor } from "@palform/palform-typescript-openapi";
    import WebauthnButton from "../../../auth/WebauthnButton.svelte";
    import { DateTime } from "luxon";

    interface Props {
        nickname: string;
        onenroll: (factor: APIAdminUserSecondAuthenticationFactor) => void;
    }

    let { nickname, onenroll }: Props = $props();

    let onEnroll = $derived((e: string) => {
        onenroll({
            id: e,
            nickname,
            created_at: DateTime.now().toISO(),
            method: "Webauthn",
        });
    });
</script>

<p>To continue, please click the button.</p>

<WebauthnButton
    flowType="register"
    registerNickname={nickname}
    onenroll={onEnroll}
/>

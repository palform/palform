<script lang="ts">
    import FormDangerDelete from "../../components/forms/settings/FormDangerDelete.svelte";
    import FormDangerMove from "../../components/forms/settings/FormDangerMove.svelte";
    import FormEndConfiguration from "../../components/forms/settings/FormEndConfiguration.svelte";
    import FormLanguage from "../../components/forms/settings/FormLanguage.svelte";
    import FormNotificationConfig from "../../components/forms/settings/FormNotificationConfig.svelte";
    import FormSettingsForm from "../../components/forms/settings/FormSettingsForm.svelte";
    import FormSubmissionAutoDelete from "../../components/forms/settings/FormSubmissionAutoDelete.svelte";
    import DangerZone from "../../components/type/DangerZone.svelte";
    import SectionSeparator from "../../components/type/SectionSeparator.svelte";
    import { getFormCtx } from "../../data/contexts/orgLayout";
    import { getResponsesContext } from "../../data/contexts/results";

    const respCtx = getResponsesContext();
    const formCtx = getFormCtx();
</script>

<FormSettingsForm
    initialValue={{
        id: $respCtx.formId,
        editor_name: $formCtx.editor_name,
        title: $formCtx.title,
        branding_id: $formCtx.branding_id,
        notification_email: $formCtx.notification_email,
        notification_webhook_url: $formCtx.notification_webhook_url,
        end_configuration: $formCtx.end_configuration,
        enable_captcha: $formCtx.enable_captcha,
    }}
    initialTeamId={$formCtx.team_id}
/>

<SectionSeparator />
<FormEndConfiguration />

<SectionSeparator />
<FormNotificationConfig />

<SectionSeparator />
<FormSubmissionAutoDelete />

<SectionSeparator />
<FormLanguage />

<SectionSeparator />
<DangerZone />
<FormDangerMove class="mt-4" />
<FormDangerDelete class="mt-4" />

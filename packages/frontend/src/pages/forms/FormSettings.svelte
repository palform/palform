<script lang="ts">
    import { useLocation } from "svelte-routing";
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

    const formCtx = getFormCtx();
    const location = useLocation();

    const selectField = $location.state.selectField as string | undefined;
</script>

<FormSettingsForm
    {selectField}
    oqpp={$formCtx.one_question_per_page}
    initialValue={{
        id: $formCtx.id,
        editor_name: $formCtx.editor_name,
        title: $formCtx.title,
        branding_id: $formCtx.branding_id,
        notification_email: $formCtx.notification_email,
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

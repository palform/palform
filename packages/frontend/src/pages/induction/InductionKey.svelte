<script lang="ts">
    import { navigate, typedNavigate } from "../../router";
    import BigAlert from "../../components/induction/bigAlert/BigAlert.svelte";
    import BigAlertHeading from "../../components/induction/bigAlert/BigAlertHeading.svelte";
    import BigAlertText from "../../components/induction/bigAlert/BigAlertText.svelte";
    import KeyRegisterForm from "../../components/keys/KeyRegisterForm.svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { APIs } from "../../data/common";
    import { getIntentTemplate } from "../../data/auth/intent";
    import type { APIForm } from "@palform/palform-typescript-openapi";

    const orgCtx = getOrgContext();
    let onKeyRegistered = $derived(async () => {
        const firstAdminTeam = $orgCtx.myTeams.find(
            (e) => e.my_role === "Admin" || e.my_role === "Editor"
        );
        if (firstAdminTeam) {
            const intentTemplate = getIntentTemplate();

            let firstForm: APIForm;
            if (intentTemplate) {
                const resp = await APIs.formTemplatesWithToken().then((a) =>
                    a.formTemplatesClone($orgCtx.org.id, intentTemplate, {
                        into_team: firstAdminTeam.team_id,
                    })
                );
                firstForm = resp.data;
            } else {
                const resp = await APIs.forms().then((a) =>
                    a.formsCreate($orgCtx.org.id, {
                        in_team: firstAdminTeam.team_id,
                        title: "My first form",
                        editor_name: "My first form",
                        one_question_per_page: true,
                    })
                );
                firstForm = resp.data;
            }

            orgCtx.update((ctx) => {
                return {
                    ...ctx,
                    forms: [firstForm, ...ctx.forms],
                };
            });

            typedNavigate("/orgs/:orgId/forms/:formId/overview", {
                params: { orgId: $orgCtx.org.id, formId: firstForm.id },
            });
        } else {
            typedNavigate("/orgs/:orgId/induction", {
                params: { orgId: $orgCtx.org.id },
            });
        }
    });
</script>

<BigAlert class="mb-8">
    <BigAlertHeading>Your recovery phrase</BigAlertHeading>
    <BigAlertText class="mt-2 leading-tight">
        Palform protects your form responses with high-security encryption.
    </BigAlertText>
    <BigAlertText class="mt-2 leading-tight">
        <strong
            >Write down these words (in this order) and save them somewhere
            safe!</strong
        >
        You'll need them whenever you sign in on a new device or if you lose your
        current one.
    </BigAlertText>
</BigAlert>

<KeyRegisterForm ondone={onKeyRegistered} showInfo={false} autoCreate />

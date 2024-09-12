<script lang="ts">
    import { navigate } from "svelte-routing";
    import BigAlert from "../../components/induction/bigAlert/BigAlert.svelte";
    import BigAlertHeading from "../../components/induction/bigAlert/BigAlertHeading.svelte";
    import BigAlertText from "../../components/induction/bigAlert/BigAlertText.svelte";
    import KeyRegisterForm from "../../components/keys/KeyRegisterForm.svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { APIs } from "../../data/common";

    const orgCtx = getOrgContext();
    $: onKeyRegistered = async () => {
        const firstAdminTeam = $orgCtx.myTeams.find(
            (e) => e.my_role === "Admin" || e.my_role === "Editor"
        );
        if (firstAdminTeam) {
            const firstFormResp = await APIs.forms().then((a) =>
                a.formsCreate($orgCtx.org.id, {
                    in_team: firstAdminTeam.team_id,
                    title: "My first form",
                    editor_name: "My first form",
                    one_question_per_page: true,
                })
            );

            orgCtx.update((ctx) => {
                return {
                    ...ctx,
                    forms: [firstFormResp.data, ...ctx.forms],
                };
            });

            navigate(`/orgs/${$orgCtx.org.id}/forms/${firstFormResp.data.id}/`);
        } else {
            navigate(`/orgs/${$orgCtx.org.id}/induction`);
        }
    };
</script>

<BigAlert class="mb-8">
    <BigAlertHeading>Set up end-to-end encryption</BigAlertHeading>
    <BigAlertText class="mt-2 leading-tight">
        Palform protects your form responses with high-security encryption.
    </BigAlertText>
    <BigAlertText class="leading-tight">
        Nobody can see your responses, except you (not even us!)
    </BigAlertText>
    <BigAlertText class="mt-2 leading-tight">
        You need to create a special <strong>key</strong> to make this work.
    </BigAlertText>
    <BigAlertText class="leading-tight">
        This will live <strong>in your browser</strong>, as well as a backup on
        our server encrypted with a secret passphrase.
    </BigAlertText>
</BigAlert>

<KeyRegisterForm on:done={onKeyRegistered} showInfo={false} autoCreate />

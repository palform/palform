<script lang="ts">
    import { navigate } from "svelte-routing";
    import BigAlert from "../../components/induction/bigAlert/BigAlert.svelte";
    import BigAlertHeading from "../../components/induction/bigAlert/BigAlertHeading.svelte";
    import BigAlertText from "../../components/induction/bigAlert/BigAlertText.svelte";
    import KeyRegisterForm from "../../components/keys/KeyRegisterForm.svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";

    const orgCtx = getOrgContext();
    $: onKeyRegistered = () => {
        navigate(`/orgs/${$orgCtx.org.id}/induction`);
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

<KeyRegisterForm on:done={onKeyRegistered} showInfo={false} />

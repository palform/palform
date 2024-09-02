<script>
    import { Button } from "flowbite-svelte";
    import { navigate } from "svelte-routing";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import BigAlert from "../../components/induction/bigAlert/BigAlert.svelte";
    import BigAlertHeading from "../../components/induction/bigAlert/BigAlertHeading.svelte";
    import BigAlertText from "../../components/induction/bigAlert/BigAlertText.svelte";

    const orgCtx = getOrgContext();
    const onContinueClick = () => {
        navigate(`/orgs/${$orgCtx.org.id}/induction/key`);
    };
</script>

<BigAlert>
    <BigAlertHeading>Amazing! Nearly done...</BigAlertHeading>
    <BigAlertText class="mt-4">
        Let's configure some basic resources before we finish.
    </BigAlertText>
    <BigAlertText>
        Remember, you'll be able to change your billing plan whenever you'd
        like.
    </BigAlertText>
    <Button color="primary" class="mt-4" size="lg" on:click={onContinueClick}>
        Continue
    </Button>
</BigAlert>

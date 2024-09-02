<script>
    import { Button } from "flowbite-svelte";
    import BigAlert from "../../components/induction/bigAlert/BigAlert.svelte";
    import BigAlertHeading from "../../components/induction/bigAlert/BigAlertHeading.svelte";
    import BigAlertText from "../../components/induction/bigAlert/BigAlertText.svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { navigate } from "svelte-routing";

    const orgCtx = getOrgContext();

    $: onGetStartedClick = () => {
        navigate(`/orgs/${$orgCtx.org.id}/induction/key`);
    };
</script>

<BigAlert>
    <BigAlertHeading>Welcome to {$orgCtx.org.display_name}!</BigAlertHeading>
    <BigAlertText class="mt-2">
        You've joined {$orgCtx.org.display_name} on Palform successfully.
    </BigAlertText>
    <BigAlertText>
        To help you get started with Palform, we'll guide you through some easy
        set-up steps.
    </BigAlertText>

    <Button size="xl" class="mt-4" on:click={onGetStartedClick}>
        Let's go!
    </Button>
</BigAlert>

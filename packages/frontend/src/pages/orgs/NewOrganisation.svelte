<script lang="ts">
    import { Alert, Button, Helper, Input, Label } from "flowbite-svelte";
    import Main from "../../layouts/Main.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faHome, faInfoCircle } from "@fortawesome/free-solid-svg-icons";
    import MainTitle from "../../layouts/MainTitle.svelte";
    import LoadingButton from "../../components/LoadingButton.svelte";
    import { APIs } from "../../data/common";
    import { showFailureToast, showSuccessToast } from "../../data/toast";
    import { navigate } from "svelte-routing";
    import { navigateEvent } from "@paltiverse/palform-frontend-common";

    let displayName = "";
    let loading = false;

    $: onFormSubmit = async (e: Event) => {
        e.preventDefault();
        loading = true;
        try {
            const resp = await APIs.orgs().then((a) =>
                a.orgsCreate({
                    display_name: displayName,
                })
            );
            await showSuccessToast("Organisation created! Have fun!");
            navigate(`/orgs/${resp.data}/induction/billing`);
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };
</script>

<Main>
    <Button size="xs" outline class="mb-2" href="/" on:click={navigateEvent}>
        <FontAwesomeIcon icon={faHome} class="me-2" />
        Back home
    </Button>
    <MainTitle>Create a new organisation</MainTitle>

    <Alert color="gray" border class="mt-4">
        <h2 class="text-lg">
            <FontAwesomeIcon icon={faInfoCircle} class="me-2" />
            Welcome to Palform!
        </h2>

        <p>We're very excited to have your organisation join our platform.</p>

        <p>
            We'll make the set-up process quick for you: just create your
            organisation, choose a plan (or stay on our limited free service),
            and you're ready!
        </p>
    </Alert>

    <form class="mt-4" on:submit={onFormSubmit}>
        <Label>
            Organisation name
            <Input class="mt-2" bind:value={displayName} disabled={loading} />
            <Helper class="mt-2">
                This will be shown to all members and users filling in your
                forms, so make sure it clearly identifies your organisation.
                You'll be able to change it whenever you want.
            </Helper>
        </Label>

        <LoadingButton
            buttonClass="mt-4"
            disabled={loading}
            {loading}
            type="submit"
        >
            Create!
        </LoadingButton>
    </form>
</Main>

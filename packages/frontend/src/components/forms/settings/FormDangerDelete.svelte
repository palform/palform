<script>
    import { Alert } from "flowbite-svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { navigate } from "svelte-routing";
    import { showSuccessToast } from "../../../data/toast";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { getFormAdminContext } from "../../../data/contexts/formAdmin";

    const orgCtx = getOrgContext();
    const formAdminCtx = getFormAdminContext();

    let loading = false;
    $: onDeleteClick = async () => {
        loading = true;
        await APIs.forms().then((a) =>
            a.formsDelete($orgCtx.org.id, $formAdminCtx.formId)
        );
        orgCtx.update((ctx) => {
            return {
                ...ctx,
                forms: ctx.forms.filter((e) => e.id !== $formAdminCtx.formId),
            };
        });
        loading = false;

        await showSuccessToast("Form deleted");
        navigate(`/orgs/${$orgCtx.org.id}`);
    };
</script>

<Alert color="red" class={$$props.class}>
    <h3 class="text-lg">Delete form</h3>
    <p>This will also delete all responses and sharing tokens.</p>
    <LoadingButton
        buttonClass="mt-2"
        color="red"
        outline
        disabled={loading}
        {loading}
        on:click={onDeleteClick}
    >
        Delete
    </LoadingButton>
</Alert>

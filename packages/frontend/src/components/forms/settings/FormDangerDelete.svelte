<script lang="ts">
    import { Alert, P } from "flowbite-svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { navigate } from "../../../router";
    import { showSuccessToast } from "../../../data/toast";
    import {
        getFormCtx,
        getOrgContext,
    } from "../../../data/contexts/orgLayout";
    import { getFormAdminContext } from "../../../data/contexts/formAdmin";
    import DestructiveModal from "../../DestructiveModal.svelte";

    interface Props {
        class?: string;
    }

    let { class: className }: Props = $props();

    const orgCtx = getOrgContext();
    const formAdminCtx = getFormAdminContext();
    const formMetadataCtx = getFormCtx();

    let showDeleteModal = $state(false);
    let onDeleteClick = $derived(async () => {
        await APIs.forms().then((a) =>
            a.formsDelete($orgCtx.org.id, $formAdminCtx.formId)
        );
        orgCtx.update((ctx) => {
            return {
                ...ctx,
                forms: ctx.forms.filter((e) => e.id !== $formAdminCtx.formId),
            };
        });

        await showSuccessToast("Form deleted");
        navigate(`/orgs/${$orgCtx.org.id}`);
    });
</script>

<DestructiveModal
    bind:show={showDeleteModal}
    targetName={$formMetadataCtx.editor_name}
    confirmationWord={$formMetadataCtx.editor_name}
    ondelete={onDeleteClick}
>
    <P>
        This will <strong>irreversibly</strong> delete this form and all of its
        <strong>{$formMetadataCtx.response_count} response(s)</strong>.
    </P>
    <P>All published links to fill your form will stop working immediately.</P>
</DestructiveModal>

<Alert color="red" class={className}>
    <h3 class="text-lg">Delete form</h3>
    <p>This will also delete all responses and sharing tokens.</p>
    <LoadingButton
        buttonClass="mt-2"
        color="red"
        outline
        onclick={() => (showDeleteModal = true)}
    >
        Delete
    </LoadingButton>
</Alert>

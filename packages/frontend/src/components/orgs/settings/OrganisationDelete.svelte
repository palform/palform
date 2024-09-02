<script lang="ts">
    import { Alert } from "flowbite-svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";

    const orgCtx = getOrgContext();
    let loading = false;
    let done = false;
    $: onDeleteClick = async () => {
        loading = true;
        try {
            await APIs.orgs().then((a) => a.orgsDelete($orgCtx.org.id));
            await showSuccessToast("Organisation deletion requested");
            done = true;
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    };
</script>

<Alert color="red" class={$$props.class}>
    <h2 class="text-lg">Delete your organisation</h2>
    {#if done}
        <p>
            Organisation deletion requested successfully. We'll send a
            confirmation email once it's all done.
        </p>
    {:else}
        <p>
            This will delete <strong>everything</strong> in your organisation, including
            all forms, responses, audit logs, etc.
        </p>
        <p>
            Your billing details (e.g. invoice and payment methods) will be
            retained as needed for us to fulfill our contractual or legal
            obligations. Any subscriptions will be cancelled with no additional
            further charge.
        </p>
        <p>
            Please click the button below to begin the process. <strong
                >A member of our team will review your case</strong
            > and get in touch if anything needs changing before your organisation
            can be deleted.
        </p>

        <LoadingButton
            buttonClass="mt-2"
            color="red"
            {loading}
            disabled={loading}
            on:click={onDeleteClick}
        >
            Delete my organisation
        </LoadingButton>
    {/if}
</Alert>

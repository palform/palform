<script lang="ts">
    import { Input } from "flowbite-svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import InfoText from "../../type/InfoText.svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";

    const orgCtx = getOrgContext();

    let value = $orgCtx.org.display_name;
    let loading = false;

    $: onRenameClick = async () => {
        loading = true;
        try {
            await APIs.orgs().then((a) =>
                a.orgsRename($orgCtx.org.id, { display_name: value })
            );
            await showSuccessToast("Organisation renamed!");
            $orgCtx.org.display_name = value;
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    };
</script>

<InfoText class="mt-2">
    This won't affect any of your organisation's resources. Your organisation's
    name is visible to people filling in your forms.
</InfoText>

<Input class="mt-4" bind:value />

{#if value !== $orgCtx.org.display_name}
    <LoadingButton
        buttonClass="mt-4"
        disabled={loading}
        {loading}
        on:click={onRenameClick}
    >
        Rename
    </LoadingButton>
{/if}

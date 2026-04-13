<script lang="ts">
    import { navigate } from "../../../router";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import KeyRegisterForm from "../../../components/keys/KeyRegisterForm.svelte";

    const orgCtx = getOrgContext();
    let onDone = $derived(() => {
        navigate(`/orgs/${$orgCtx.org.id}/user/keys`);
    });
</script>

<KeyRegisterForm ondone={onDone} />

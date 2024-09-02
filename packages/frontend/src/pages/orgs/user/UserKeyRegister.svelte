<script lang="ts">
    import { navigate } from "svelte-routing";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import KeyRegisterForm from "../../../components/keys/KeyRegisterForm.svelte";

    const orgCtx = getOrgContext();
    $: onDone = () => {
        navigate(`/orgs/${$orgCtx.org.id}/user/keys`);
    };
</script>

<KeyRegisterForm on:done={onDone} />

<script lang="ts">
    import { navigate } from "svelte-routing";
    import { getFormCtx, getOrgContext } from "../../data/contexts/orgLayout";
    import MainTitle from "../../layouts/MainTitle.svelte";

    const orgCtx = getOrgContext();
    const formCtx = getFormCtx();

    const onClick = () => {
        navigate(`/orgs/${$orgCtx.org.id}/forms/${$formCtx.id}/settings`, {
            state: {
                selectField: "editor_name",
            },
        });
    };
</script>

<MainTitle className={$$props.class}>
    <button
        class="outline-offset-4 outline-gray-200 outline-2 hover:outline rounded-lg cursor-text"
        on:click={onClick}
    >
        {$formCtx.editor_name}
    </button>
</MainTitle>

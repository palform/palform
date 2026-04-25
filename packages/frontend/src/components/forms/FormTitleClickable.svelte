<script lang="ts">
    import { navigate } from "../../router";
    import { getFormCtx, getOrgContext } from "../../data/contexts/orgLayout";
    import MainTitle from "../../layouts/MainTitle.svelte";
    interface Props {
        [key: string]: any;
    }

    let { ...props }: Props = $props();

    const orgCtx = getOrgContext();
    const formCtx = getFormCtx();

    let onClick = $derived(() => {
        navigate(`/orgs/${$orgCtx.org.id}/forms/${$formCtx.id}/settings`, {
            state: {
                selectField: "editor_name",
            },
        });
    });
</script>

<MainTitle className={props.class}>
    <button
        class="outline-offset-4 outline-gray-200 hover:outline-2 rounded-lg cursor-text!"
        onclick={onClick}
    >
        {$formCtx.editor_name}
    </button>
</MainTitle>

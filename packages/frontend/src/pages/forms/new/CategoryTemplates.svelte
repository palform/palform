<script lang="ts">
    import FormTemplateList from "./FormTemplateList.svelte";

    import { route } from "../../../router";

    interface Props {
        categoryId?: string | undefined;
    }

    let { categoryId }: Props = $props();

    const categoryIdResolved = $derived(categoryId ?? route.params.categoryId ?? "");
</script>

<FormTemplateList source={categoryIdResolved} />

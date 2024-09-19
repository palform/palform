<script lang="ts">
    import FormTemplateList from "./FormTemplateList.svelte";

    export let categoryId: string;
</script>

<FormTemplateList source={categoryId} />

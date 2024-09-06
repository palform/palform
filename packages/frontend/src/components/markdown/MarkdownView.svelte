<script lang="ts">
    import SvelteMarkdown from "svelte-markdown";
    import MarkdownLink from "./MarkdownLink.svelte";
    import MarkdownFormFillImage from "./MarkdownFormFillImage.svelte";

    export let value = "";
    export let imagesWithFillToken = false;
</script>

<div class="space-y-2">
    <SvelteMarkdown
        source={value}
        renderers={{
            link: MarkdownLink,
            image: imagesWithFillToken ? MarkdownFormFillImage : undefined,
        }}
        options={{ mangle: false }}
    />
</div>

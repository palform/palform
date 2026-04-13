<script lang="ts">
    import MarkdownLink from "./MarkdownLink.svelte";
    import MarkdownFormFillImage from "./MarkdownFormFillImage.svelte";
    import MarkdownList from "./MarkdownList.svelte";
    import SvelteMarkdown from "@humanspeak/svelte-markdown";

    interface Props {
        value?: string;
        imagesWithFillToken?: boolean;
    }

    let { value = "", imagesWithFillToken = false }: Props = $props();
</script>

<div class="space-y-2">
    <SvelteMarkdown
        source={value}
        renderers={{
            link: MarkdownLink,
            image: imagesWithFillToken ? MarkdownFormFillImage : undefined,
            list: MarkdownList,
        }}
    />
</div>

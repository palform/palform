<script lang="ts">
    import {
        ButtonGroup,
        Input,
        InputAddon,
        Label,
        NumberInput,
        TabItem,
        Tabs,
    } from "flowbite-svelte";
    import {
        formatFillTokenURL,
        formatShortLinkURL,
    } from "../../../data/fillTokens";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { getResponsesContext } from "../../../data/contexts/results";
    import { copyGenericValue } from "../../../data/util/clipboard";

    export let fatID: string;
    export let shortLink: string | undefined;

    const orgCtx = getOrgContext();
    const respCtx = getResponsesContext();

    $: longURL = formatFillTokenURL($orgCtx.org.id, $respCtx.formId, fatID);

    let frameHeight = 800;
    let frameWidth = 600;
    $: frameCode = `<iframe src="${longURL}" height="${frameHeight}px" width="${frameWidth}px" />`;

    $: onInputCopy = async (e: Event) => {
        const t = e.target as HTMLInputElement;
        await copyGenericValue(t.value);
    };
</script>

<div>
    <Tabs contentClass="mt-4">
        <TabItem open title="Link">
            <Label>
                Click to copy URL
                <Input
                    class="mt-2"
                    readonly
                    value={longURL}
                    on:click={onInputCopy}
                />
            </Label>

            {#if shortLink && $orgCtx.org.subdomain}
                <Label class="mt-4">
                    Click to copy short link
                    <Input
                        class="mt-2"
                        readonly
                        value={formatShortLinkURL(
                            $orgCtx.org.subdomain,
                            shortLink
                        )}
                        on:click={onInputCopy}
                    />
                </Label>
            {/if}
        </TabItem>

        <TabItem title="Embed">
            <div class="flex gap-4">
                <Label>
                    Frame width
                    <ButtonGroup class="mt-2 flex">
                        <NumberInput bind:value={frameWidth} />
                        <InputAddon>px</InputAddon>
                    </ButtonGroup>
                </Label>
                <Label>
                    Frame height
                    <ButtonGroup class="mt-2 flex">
                        <NumberInput bind:value={frameHeight} />
                        <InputAddon>px</InputAddon>
                    </ButtonGroup>
                </Label>
            </div>

            <Label class="mt-4">
                Click to copy embed code
                <Input
                    class="mt-2"
                    readonly
                    value={frameCode}
                    on:click={onInputCopy}
                />
            </Label>
        </TabItem>
    </Tabs>
</div>

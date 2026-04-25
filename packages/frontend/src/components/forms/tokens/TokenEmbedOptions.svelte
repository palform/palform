<script lang="ts">
    import {
        ButtonGroup,
        Input,
        InputAddon,
        Label,
        TabItem,
        Tabs,
    } from "flowbite-svelte";
    import {
        formatFillTokenURL,
        formatShortLinkURL,
    } from "../../../data/fillTokens";
    import {
        getFormCtx,
        getOrgContext,
    } from "../../../data/contexts/orgLayout";
    import { copyGenericValue } from "../../../data/util/clipboard";
    import { getFormAdminContext } from "../../../data/contexts/formAdmin";
    import QrCode from "../../QRCode.svelte";
    import HiddenHelp from "../../HiddenHelp.svelte";
    import InfoText from "../../type/InfoText.svelte";

    interface Props {
        fatID: string;
        shortLink: string | undefined;
    }

    let { fatID, shortLink }: Props = $props();

    const orgCtx = getOrgContext();
    const formAdminCtx = getFormAdminContext();
    const formCtx = getFormCtx();

    let longURL = $state("Loading...");
    $effect(() => {
        (async () => {
            longURL = await formatFillTokenURL(
                $orgCtx.org.id,
                $formCtx.team_id,
                $formAdminCtx.formId,
                fatID
            );
        })();
    });

    let frameHeight = $state(800);
    let frameWidth = $state(600);
    let frameCode = $derived(
        `<iframe src="${longURL}" height="${frameHeight}px" width="${frameWidth}px" />`
    );

    let onInputCopy = $derived(async (e: Event) => {
        const t = e.target as HTMLInputElement;
        await copyGenericValue(t.value);
    });
</script>

<div>
    <Tabs classes={{ content: "mt-4 p-0 bg-transparent" }}>
        <TabItem open title="Link">
            <Label>
                Click to copy URL
                <Input
                    class="mt-2"
                    readonly
                    value={longURL}
                    onclick={onInputCopy}
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
                        onclick={onInputCopy}
                    />
                </Label>
            {/if}
        </TabItem>

        <TabItem title="QR code">
            <QrCode uri={longURL} download />
        </TabItem>

        <TabItem title="Embed">
            <div class="flex gap-4">
                <Label>
                    Frame width
                    <ButtonGroup class="mt-2 flex">
                        <Input type="number" bind:value={frameWidth} />
                        <InputAddon>px</InputAddon>
                    </ButtonGroup>
                </Label>
                <Label>
                    Frame height
                    <ButtonGroup class="mt-2 flex">
                        <Input type="number" bind:value={frameHeight} />
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
                    onclick={onInputCopy}
                />
            </Label>
        </TabItem>
    </Tabs>

    <HiddenHelp class="mt-2" href="https://docs.palform.app/keys/integrity">
        <InfoText>
            The default long URL of this link contains a list of key
            fingerprints to allow for <strong>key integrity checking</strong>,
            an important part of Palform's security model.
        </InfoText>
        <InfoText>
            If you add more users or keys to this form, you'll need to re-copy
            and re-distribute this link if you want the new user(s) to have
            access to new responses. You <em>won't</em> have to re-create the Share
            Token itself.
        </InfoText>
    </HiddenHelp>
</div>

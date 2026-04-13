<script lang="ts">
    import {
        Textarea,
        Toolbar,
        ToolbarButton,
        ToolbarGroup,
    } from "flowbite-svelte";
    import TeamAssetModal from "../teams/assets/TeamAssetModal.svelte";
    import {
        faBold,
        faImage,
        faItalic,
    } from "@fortawesome/free-solid-svg-icons";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { backendURL } from "../../data/common";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";

    interface Props {
        value?: string;
        id?: string;
        disabled?: boolean;
        imageFormId?: string;
        imageTeamId?: string;
        class?: string;
    }

    let {
        value = $bindable(""),
        id = undefined,
        disabled = false,
        imageFormId = undefined,
        imageTeamId = undefined,
        class: className,
    }: Props = $props();

    const orgCtx = getOrgContext();

    let textareaRef: HTMLTextAreaElement | undefined = $state();
    let touched = $state(false);

    function applyAction(wrap: string) {
        textareaRef?.focus();
        if (!textareaRef) return;
        if (touched) {
            if (textareaRef.selectionStart === textareaRef.selectionEnd) {
                value = [
                    value.slice(0, textareaRef.selectionStart),
                    wrap + wrap,
                    value.slice(textareaRef.selectionStart),
                ].join("");
            } else {
                value = [
                    value.slice(0, textareaRef.selectionStart),
                    wrap,
                    value.slice(
                        textareaRef.selectionStart,
                        textareaRef.selectionEnd
                    ),
                    wrap,
                    value.slice(textareaRef.selectionEnd),
                ].join("");
            }
        } else {
            value += wrap + wrap;
        }
    }

    let showImageModal = $state(false);
    function onStartImageSelect() {
        showImageModal = true;
    }
    let loading = $state(false);
    function onFileSelect(e: string | null) {
        if (!imageFormId || !e || !textareaRef) return;

        showImageModal = false;

        value = [
            value.slice(0, textareaRef.selectionStart),
            `![caption](${backendURL}/fill/orgs/${$orgCtx.org.id}/forms/${imageFormId}/assets/${e}?f={{token}})`,
            value.slice(textareaRef.selectionStart),
        ].join("");
    }
</script>

<fieldset class={`block ${className ?? ""}`}>
    <Textarea
        class="rounded-t-none"
        {id}
        bind:value
        bind:elementRef={textareaRef}
        onfocus={() => (touched = true)}
        disabled={disabled || loading}
    >
        {#snippet header()}
            <Toolbar embedded>
                <ToolbarGroup>
                    <ToolbarButton
                        name="Make text bold"
                        disabled={disabled || loading}
                        onclick={() => applyAction("**")}
                    >
                        <FontAwesomeIcon icon={faBold} />
                    </ToolbarButton>
                    <ToolbarButton
                        name="Italicise text"
                        disabled={disabled || loading}
                        onclick={() => applyAction("_")}
                    >
                        <FontAwesomeIcon icon={faItalic} />
                    </ToolbarButton>
                    <ToolbarButton
                        name="Attach an image"
                        disabled={disabled || loading}
                        onclick={onStartImageSelect}
                    >
                        <FontAwesomeIcon icon={faImage} />
                    </ToolbarButton>
                </ToolbarGroup>
            </Toolbar>
        {/snippet}
    </Textarea>
    <p class="text-xs mt-1 text-gray-500 dark:text-gray-400">
        Supports <a
            href="https://www.markdownguide.org/cheat-sheet/"
            target="_blank"
            class="underline">Markdown</a
        >. Drag & drop images to insert.
    </p>
</fieldset>

{#if imageTeamId}
    <TeamAssetModal
        bind:show={showImageModal}
        teamId={imageTeamId}
        onselect={onFileSelect}
    />
{/if}

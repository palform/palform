<script lang="ts">
    import { Spinner } from "flowbite-svelte";
    import TeamAssetModal from "../teams/assets/TeamAssetModal.svelte";
    import MarkdownEditorControl from "./MarkdownEditorControl.svelte";
    import {
        faBold,
        faImage,
        faItalic,
    } from "@fortawesome/free-solid-svg-icons";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { backendURL } from "../../data/common";

    export let value = "";
    export let id: string | undefined = undefined;
    export let disabled = false;
    export let imageFormId: string | undefined = undefined;
    export let imageTeamId: string | undefined = undefined;
    const orgCtx = getOrgContext();

    let textareaRef: HTMLTextAreaElement;
    let touched = false;

    $: applyAction = (wrap: string) => {
        textareaRef.focus();
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
    };

    let showImageModal = false;
    $: onStartImageSelect = () => {
        showImageModal = true;
    };
    let loading = false;
    $: onFileSelect = (e: CustomEvent<string | null>) => {
        if (!imageFormId || !e.detail) return;

        showImageModal = false;

        value = [
            value.slice(0, textareaRef.selectionStart),
            `![caption](${backendURL}/fill/orgs/${$orgCtx.org.id}/forms/${imageFormId}/assets/${e.detail}?f={{token}})`,
            value.slice(textareaRef.selectionStart),
        ].join("");
    };
</script>

<fieldset class={`block ${$$props.class}`}>
    <div
        class="w-full px-4 bg-white border border-b-0 rounded-t-lg flex justify-between items-center"
    >
        <div class="space-x-1">
            <MarkdownEditorControl
                icon={faBold}
                on:click={() => applyAction("**")}
                disabled={disabled || loading}
            /><MarkdownEditorControl
                icon={faItalic}
                on:click={() => applyAction("_")}
                disabled={disabled || loading}
            />{#if imageTeamId}<MarkdownEditorControl
                    icon={faImage}
                    on:click={onStartImageSelect}
                    disabled={disabled || loading}
                />{/if}
        </div>
        {#if loading}
            <div>
                <Spinner size={6} />
            </div>
        {/if}
    </div>
    <textarea
        class="w-full rounded-lg rounded-t-none bg-gray-50 dark:bg-gray-700 text-gray-900 dark:placeholder-gray-400 dark:text-white border dark:border-gray-600 p-2.5 text-sm focus:ring-primary-500 border-gray-300 focus:border-primary-500 dark:focus:ring-primary-500 dark:focus:border-primary-500 disabled:cursor-not-allowed disabled:opacity-50"
        {id}
        bind:value
        bind:this={textareaRef}
        on:focus={() => (touched = true)}
        disabled={disabled || loading}
    />
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
        on:select={onFileSelect}
    />
{/if}

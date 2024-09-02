<script lang="ts">
    import type { QuestionSubmissionData } from "@paltiverse/palform-client-js-extra-types/QuestionSubmissionData";
    import type { APIQuestionConfigurationOneOf7 } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher } from "svelte";
    import PaintCanvas from "../../paint/PaintCanvas.svelte";
    import InfoText from "../../type/InfoText.svelte";
    import { Button, Input, Label, Modal } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faA,
        faICursor,
        faSignature,
    } from "@fortawesome/free-solid-svg-icons";
    import {
        setQuestionValue,
        sGetSignature,
    } from "../../../data/contexts/fill";
    import TextButton from "../../TextButton.svelte";
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../../data/contexts/brand";
    import BrandedButton from "../../teams/brandings/BrandedButton.svelte";

    export let id: string;
    export let config: APIQuestionConfigurationOneOf7;
    export let currentValue: QuestionSubmissionData | undefined;
    $: value = currentValue
        ? sGetSignature(currentValue)
        : { freeform: [], initial: "", full_name: "" };

    const dispatch = createEventDispatcher<{ change: undefined }>();
    const brandCtx = getBrandCtx();

    $: supportedMethodCount = Object.values(config.signature).filter(
        (e) => e === true
    ).length;
    $: onlyOne = supportedMethodCount === 1;
    let selectedMethod: "freeform" | "initial" | "full_name" | null = null;

    $: selectMethod = (method: "freeform" | "initial" | "full_name") => {
        selectedMethod = method;
    };

    $: onUpdateFreeform = (e: CustomEvent<number[][][]>) => {
        setQuestionValue(id, {
            Signature: {
                freeform: e.detail,
                initial: "",
                full_name: "",
            },
        });
        dispatch("change");
    };

    $: onUpdateInitials = (e: Event) => {
        const t = e.target as HTMLInputElement;
        setQuestionValue(id, {
            Signature: {
                freeform: [],
                initial: t.value.toUpperCase(),
                full_name: "",
            },
        });
        dispatch("change");
    };

    $: onUpdateFullName = (e: Event) => {
        const t = e.target as HTMLInputElement;
        setQuestionValue(id, {
            Signature: {
                freeform: [],
                initial: "",
                full_name: t.value,
            },
        });
        dispatch("change");
    };

    $: onClear = () => {
        setQuestionValue(id, {
            Signature: {
                freeform: [],
                initial: "",
                full_name: "",
            },
        });
        selectedMethod = null;
        dispatch("change");
    };

    const isTouchScreen = window.matchMedia("(pointer: coarse)").matches;
    let modalOpen = false;
    $: {
        if (modalOpen) {
            document.body.classList.add("lock");
        } else {
            document.body.classList.remove("lock");
        }
    }
</script>

{#if (onlyOne && config.signature.allow_initial) || selectedMethod === "initial" || value.initial.length > 0}
    <Label>
        Initials
        <Input class="mt-1" value={value.initial} on:input={onUpdateInitials} />
    </Label>

    {#if value.initial}
        <p
            class="mt-3 text-4xl font-medium font-mono tracking-widest dark:text-gray-300"
        >
            {value.initial}
        </p>
    {/if}
{:else if (onlyOne && config.signature.allow_freeform) || selectedMethod === "freeform" || value.freeform.length > 0}
    {#if isTouchScreen}
        <button
            class="h-32 border-2 dark:border-gray-700 w-full text-gray-600 dark:text-gray-400"
            style:border-radius={getRoundingAmountForBrand($brandCtx)}
            on:click={() => (modalOpen = true)}
            type="button"
        >
            {#if value.freeform.length > 0}
                Signed! Tap to edit
            {:else}
                Tap to sign
            {/if}
        </button>

        <Modal bind:open={modalOpen} outsideclose title="Draw signature">
            <PaintCanvas points={value.freeform} on:update={onUpdateFreeform} />

            <BrandedButton outline on:click={() => (modalOpen = false)}>
                Done
            </BrandedButton>
        </Modal>
    {:else}
        <PaintCanvas points={value.freeform} on:update={onUpdateFreeform} />
    {/if}
{:else if (onlyOne && config.signature.allow_full_name) || selectedMethod === "full_name" || value.full_name.length > 0}
    <Label>
        Full name
        <Input
            class="mt-1"
            value={value.full_name}
            on:input={onUpdateFullName}
        />
    </Label>
{:else if selectedMethod === null}
    <InfoText>How would you like to sign?</InfoText>

    <div
        class={`mt-2 w-full grid ${supportedMethodCount === 2 ? "grid-cols-2" : "grid-cols-3"} gap-3`}
    >
        {#if config.signature.allow_initial}
            <Button color="light" on:click={() => selectMethod("initial")}>
                <div>
                    <FontAwesomeIcon icon={faA} size="xl" class="mb-1" />
                    <span class="block">Initial</span>
                </div>
            </Button>
        {/if}
        {#if config.signature.allow_freeform}
            <Button color="light" on:click={() => selectMethod("freeform")}>
                <div>
                    <FontAwesomeIcon
                        icon={faSignature}
                        size="xl"
                        class="mb-1"
                    />
                    <span class="block">Draw</span>
                </div>
            </Button>
        {/if}
        {#if config.signature.allow_full_name}
            <Button color="light" on:click={() => selectMethod("full_name")}>
                <div>
                    <FontAwesomeIcon icon={faICursor} size="xl" class="mb-1" />
                    <span class="block">Full name</span>
                </div>
            </Button>
        {/if}
    </div>
{/if}

{#if value.initial.length > 0 || value.freeform.length > 0 || value.full_name.length > 0 || selectedMethod !== null}
    <TextButton
        class="mt-2 text-xs !text-gray-600 !dark:text-gray-400"
        on:click={onClear}
    >
        Clear
    </TextButton>
{/if}

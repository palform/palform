<script lang="ts">
    import { Button, Input, Label, Modal, P, Select } from "flowbite-svelte";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import type {
        APILegalAgreement,
        APILegalAgreementVariable,
        LegalAgreementTypeEnum,
    } from "@palform/palform-typescript-openapi";
    import SkeletonPrimitive from "../../SkeletonPrimitive.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowUpRightFromSquare } from "@fortawesome/free-solid-svg-icons";

    interface Props {
        open: boolean;
        agreementTypes: Record<string, string>;
        onNewAgreement: (newAgreement: APILegalAgreement) => void;
    }

    let {
        open = $bindable(),
        agreementTypes,
        onNewAgreement,
    }: Props = $props();

    const orgCtx = getOrgContext();
    let step = $state(0);
    let selectedAgreementType = $state("");

    let variables = $state<APILegalAgreementVariable[]>([]);
    let variablesLoading = $state(true);
    let variableValues = $state<Record<string, string>>({});

    $effect(() => {
        if (selectedAgreementType === "") return;

        variablesLoading = true;
        APIs.legalAgreements()
            .then((a) =>
                a.legalAgreementsListVariables(
                    selectedAgreementType as LegalAgreementTypeEnum,
                    $orgCtx.org.id
                )
            )
            .then((resp) => {
                variables = resp.data;
                variableValues = {};
                for (const variable of variables) {
                    variableValues[variable.id] = "";
                }
            })
            .catch(showFailureToast)
            .finally(() => (variablesLoading = false));
    });

    let submitLoading = $state(false);
    let newAgreement = $state<APILegalAgreement | null>(null);
    let onGenerate = $derived(() => {
        if (
            selectedAgreementType === "" ||
            Object.values(variableValues).some((e) => !e)
        ) {
            showFailureToast("Please complete all fields.");
            return;
        }

        submitLoading = true;
        APIs.legalAgreements()
            .then((a) =>
                a.legalAgreementsCreate(
                    selectedAgreementType as LegalAgreementTypeEnum,
                    $orgCtx.org.id,
                    {
                        values: variableValues,
                    }
                )
            )
            .then((resp) => {
                onNewAgreement(resp.data);
                newAgreement = resp.data;
                step = 2;
            })
            .catch(showFailureToast)
            .finally(() => (submitLoading = false));
    });

    $effect(() => {
        if (open) return;

        step = 0;
        newAgreement = null;
        variableValues = {};
        selectedAgreementType = "";
    });

    let onContinue = $derived(() => {
        if (step === 0) {
            if (selectedAgreementType === "") return;
            step = 1;
        } else if (step === 1) {
            onGenerate();
        } else if (step === 2) {
            open = false;
        }
    });
</script>

<Modal bind:open title="Create new legal agreement">
    {#if step === 0}
        <Label>
            Which type of agreement would you like to create?
            <Select
                class="mt-1"
                bind:value={selectedAgreementType}
                items={Object.keys(agreementTypes).map((at) => ({
                    value: at,
                    name: agreementTypes[at],
                }))}
            />
        </Label>
    {:else if step === 1}
        <P>Please complete these fields to generate your agreement.</P>

        <div>
            {#if variablesLoading}
                <SkeletonPrimitive height="10px" className="mb-2" />
                <SkeletonPrimitive height="40px" className="mb-4" />
                <SkeletonPrimitive height="10px" className="mb-2" />
                <SkeletonPrimitive height="40px" className="mb-4" />
                <SkeletonPrimitive height="10px" className="mb-2" />
                <SkeletonPrimitive height="40px" />
            {:else}
                <form>
                    {#each variables as variable (variable.id)}
                        <Label class="mb-2" for={variable.id}>
                            {variable.name}
                            <Input
                                bind:value={variableValues[variable.id]}
                                class="mt-1"
                                required
                                name={variable.id}
                                id={variable.id}
                                disabled={submitLoading}
                            />
                        </Label>
                    {/each}
                </form>
            {/if}
        </div>
    {:else if step === 2}
        <P>
            Your agreement has been created! Click the button below to view it.
        </P>
        <P>
            Please ensure you sign the agreement. You do not need to send us a
            signed copy.
        </P>
        <Button href={newAgreement?.document_url} target="_blank">
            <FontAwesomeIcon icon={faArrowUpRightFromSquare} class="me-2" />
            View agreement
        </Button>
    {/if}

    {#snippet footer()}
        <Button
            onclick={onContinue}
            disabled={submitLoading}
            loading={submitLoading}
        >
            {#if step === 0}
                Continue
            {:else if step === 1}
                Generate
            {:else if step === 2}
                Close
            {/if}
        </Button>
    {/snippet}
</Modal>

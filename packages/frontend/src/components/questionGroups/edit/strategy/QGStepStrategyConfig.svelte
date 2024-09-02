<script lang="ts">
    import { Label, Select } from "flowbite-svelte";
    import {
        ctxGetGroup,
        ctxUpdateGroup,
        getResponsesContext,
        qgsIsJump,
        syncGroupConfig,
    } from "../../../../data/contexts/results";
    import { getOrgContext } from "../../../../data/contexts/orgLayout";
    import type {
        APIQuestionGroupStepStrategy,
        APIQuestionGroupStepStrategyJumpCase,
    } from "@paltiverse/palform-typescript-openapi";
    import QgStepStrategyCase from "./QGStepStrategyCase.svelte";
    import QgStepStrategyNewCase from "./QGStepStrategyNewCase.svelte";
    import { slide } from "svelte/transition";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faChevronDown,
        faChevronRight,
    } from "@fortawesome/free-solid-svg-icons";
    import { getEditorCtx } from "../../../../data/contexts/questionsEditing";
    import InfoText from "../../../type/InfoText.svelte";

    export let groupId: string;
    const orgCtx = getOrgContext();
    const formCtx = getResponsesContext();
    const editorCtx = getEditorCtx();
    $: group = ctxGetGroup(groupId);
    $: currentConfig = $group?.step_strategy;

    $: currentActionValue =
        currentConfig !== undefined && qgsIsJump(currentConfig)
            ? "JumpToSection"
            : "NextPosition";

    $: onChange = async () => {
        await syncGroupConfig(formCtx, $orgCtx.org.id, groupId);
    };

    let showJumpCases = false;
    $: onActionValueChange = async (e: Event) => {
        if (!$group) return;
        const t = e.target as HTMLSelectElement;
        let strategy: APIQuestionGroupStepStrategy;
        if (t.value === "NextPosition") {
            strategy = "NextPosition";
        } else {
            strategy = {
                JumpToSection: [],
            };
            showJumpCases = true;
        }

        ctxUpdateGroup(formCtx, groupId, {
            ...$group,
            step_strategy: strategy,
        });
        await onChange();
    };

    $: onNewJumpCase = async (
        e: CustomEvent<APIQuestionGroupStepStrategyJumpCase>
    ) => {
        if (!$group || !currentConfig || !qgsIsJump(currentConfig)) return;
        ctxUpdateGroup(formCtx, groupId, {
            ...$group,
            step_strategy: {
                JumpToSection: [...currentConfig.JumpToSection, e.detail],
            },
        });
        await onChange();
    };

    $: onDeleteJumpCase = async (index: number) => {
        if (!$group || !currentConfig || !qgsIsJump(currentConfig)) return;
        currentConfig.JumpToSection.splice(index, 1);
        ctxUpdateGroup(formCtx, groupId, {
            ...$group,
            step_strategy: currentConfig,
        });
        await onChange();
    };
</script>

{#if currentConfig !== undefined}
    <Label class={$$props.class}>
        Action
        <Select
            class="mt-1"
            size="sm"
            value={currentActionValue}
            on:change={onActionValueChange}
            disabled={!!$editorCtx.currentlyEditing}
            items={[
                { name: "Go to next section", value: "NextPosition" },
                {
                    name: "Jump to section / submit form",
                    value: "JumpToSection",
                },
            ]}
        />
    </Label>

    {#if qgsIsJump(currentConfig)}
        <button
            class="mt-2 text-sm text-slate-700 dark:text-slate-300"
            on:click={() => (showJumpCases = !showJumpCases)}
            disabled={!!$editorCtx.currentlyEditing}
        >
            <span class="inline-block w-4 me-1">
                {#if showJumpCases}
                    <FontAwesomeIcon icon={faChevronDown} />
                {:else}
                    <FontAwesomeIcon icon={faChevronRight} />
                {/if}
            </span>
            Configure jumping
        </button>
        {#if showJumpCases && !$editorCtx.currentlyEditing}
            <div transition:slide>
                {#if currentConfig.JumpToSection.length > 0}
                    <div class="mt-4 space-y-2">
                        {#each currentConfig.JumpToSection as strategyCase, index (`${strategyCase.target_group_id}-${index}`)}
                            <QgStepStrategyCase
                                {strategyCase}
                                on:delete={() => onDeleteJumpCase(index)}
                            />

                            {#if index < currentConfig.JumpToSection.length - 1}
                                <InfoText lighter>else</InfoText>
                            {/if}
                        {/each}
                        <InfoText lighter>
                            else: <strong>submit form</strong>
                        </InfoText>
                    </div>
                {:else}
                    <InfoText lighter class="mt-2">
                        Currently (since there are no cases) the action will
                        always be <strong>Submit form</strong>.
                    </InfoText>
                {/if}

                <QgStepStrategyNewCase
                    class="mt-2"
                    fromGroupId={groupId}
                    on:saveNew={onNewJumpCase}
                />
            </div>
        {/if}
    {/if}
{/if}

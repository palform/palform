<script lang="ts">
    import { createSortable } from "@dnd-kit/svelte/sortable";
    import { faGripVertical, faTrash } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button, Input } from "flowbite-svelte";
    import { getFormEditorCtx } from "../../../data/contexts/formEditor";

    interface Props {
        option: string;
        index: number;
        questionId: string;
        onremove: () => void;
        value: string;
    }
    let {
        option,
        index,
        questionId,
        onremove,
        value = $bindable(),
    }: Props = $props();

    const sortable = createSortable({
        get id() {
            return option;
        },
        get index() {
            return index;
        },
        get group() {
            return questionId;
        },
    });

    let ctx = getFormEditorCtx();
</script>

<div class="flex gap-x-2" {@attach sortable.attach}>
    <button
        class="w-8 bg-slate-200 dark:bg-slate-700 flex justify-center items-center rounded-md cursor-grab"
        {@attach sortable.attachHandle}
    >
        <FontAwesomeIcon
            icon={faGripVertical}
            class="text-slate-700 dark:text-slate-300 "
        />
    </button>
    <Input bind:value disabled={$ctx.loading} class="flex-1" />
    {#if index !== 0}
        <Button disabled={$ctx.loading} onclick={() => onremove()}>
            <FontAwesomeIcon icon={faTrash} />
        </Button>
    {/if}
</div>

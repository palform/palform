<script lang="ts">
    import { createSortable } from "@dnd-kit/svelte/sortable";
    import { fillSendStore } from "../../../data/contexts/fill";
    import QFChoiceLabelButton from "./QFChoiceLabelButton.svelte";
    import {
        faArrowDown,
        faArrowUp,
        faGripVertical,
    } from "@fortawesome/free-solid-svg-icons";
    import { Button, ButtonGroup } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";

    interface Props {
        option: string;
        questionId: string;
        index: number;
        optionsCount: number;
        onMove: (direction: "up" | "down") => void;
    }

    let { option, questionId, index, optionsCount, onMove }: Props = $props();

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
        disabled: $fillSendStore?.loading,
        transition: {
            idle: true,
        },
    });
</script>

<li class="flex gap-x-2" {@attach sortable.attach}>
    <QFChoiceLabelButton
        {questionId}
        {option}
        isMulti={false}
        isActive={false}
        icon={faGripVertical}
        class="flex-1"
    >
        <ButtonGroup>
            <Button
                disabled={index === 0 || $fillSendStore?.loading}
                onclick={() => onMove("up")}
            >
                <FontAwesomeIcon icon={faArrowUp} />
            </Button>
            <Button
                disabled={index === optionsCount - 1 || $fillSendStore?.loading}
                onclick={() => onMove("down")}
            >
                <FontAwesomeIcon icon={faArrowDown} />
            </Button>
        </ButtonGroup>
    </QFChoiceLabelButton>
</li>

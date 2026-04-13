<script lang="ts">
    import {
        faArrowLeft,
        faBackwardFast,
        faArrowRight,
        faForwardFast,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        Button,
        ButtonGroup,
        Modal,
        Table,
        TableBody,
        TableBodyCell,
        TableBodyRow,
    } from "flowbite-svelte";
    import { DateTime } from "luxon";
    import { parseServerTime } from "../../../../../data/util/time";
    import VirtualList from "svelte-tiny-virtual-list";
    import TableContainer from "../../../../tables/TableContainer.svelte";
    import { getFormAdminContext } from "../../../../../data/contexts/formAdmin";

    interface Props {
        currentIndex?: number;
    }

    let { currentIndex = $bindable(0) }: Props = $props();
    const formAdminCtx = getFormAdminContext();

    let maxIndex = $derived($formAdminCtx.submissions.length - 1);
    const next = () => {
        currentIndex++;
    };
    const prev = () => {
        currentIndex--;
    };
    const first = () => {
        currentIndex = 0;
    };
    let last = $derived(() => {
        currentIndex = maxIndex;
    });

    let showIndexSelectModal = $state(false);
    const openIndexSelectModal = () => {
        showIndexSelectModal = true;
    };
    const jump = (index: number) => () => {
        currentIndex = index;
        showIndexSelectModal = false;
    };
</script>

<div class="w-full flex items-center justify-center mb-8">
    <div>
        <ButtonGroup>
            <Button disabled={currentIndex === 0} onclick={first}>
                <FontAwesomeIcon icon={faBackwardFast} class="mr-2" />
                First
            </Button>
            <Button disabled={currentIndex === 0} onclick={prev}>
                <FontAwesomeIcon icon={faArrowLeft} class="mr-2" />
                Prev
            </Button>
            <Button onclick={openIndexSelectModal}>
                <span class="text-base">{currentIndex + 1}</span><span
                    class="text-gray-500 text-sm">/{maxIndex + 1}</span
                >
            </Button>
            <Button disabled={currentIndex === maxIndex} onclick={next}>
                Next
                <FontAwesomeIcon icon={faArrowRight} class="ml-2" />
            </Button>
            <Button disabled={currentIndex === maxIndex} onclick={last}>
                Last
                <FontAwesomeIcon icon={faForwardFast} class="ml-2" />
            </Button>
        </ButtonGroup>
    </div>
</div>

<Modal bind:open={showIndexSelectModal} title="Jump to response" outsideclose>
    <TableContainer>
        <Table striped>
            <TableBody>
                <VirtualList
                    height={300}
                    itemCount={$formAdminCtx.submissions.length}
                    itemSize={75}
                    scrollToIndex={currentIndex}
                >
                    {#snippet item({ index, style })}
                        <TableBodyRow {style}>
                            <TableBodyCell>
                                <span
                                    class={`text-lg ${index === currentIndex ? "font-bold" : ""}`}
                                >
                                    {index + 1}
                                </span>
                            </TableBodyCell>
                            <TableBodyCell>
                                {parseServerTime(
                                    $formAdminCtx.submissions[index].createdAt
                                ).toLocaleString(DateTime.DATETIME_MED)}
                            </TableBodyCell>
                            <TableBodyCell>
                                <Button
                                    outline
                                    onclick={jump(index)}
                                    disabled={index === currentIndex}
                                >
                                    Jump
                                </Button>
                            </TableBodyCell>
                        </TableBodyRow>
                    {/snippet}
                </VirtualList>
            </TableBody>
        </Table>
    </TableContainer>
</Modal>

<script lang="ts">
    import { Button, Label, Textarea } from "flowbite-svelte";
    import SectionHeading from "../type/SectionHeading.svelte";
    import LoadingButton from "../LoadingButton.svelte";
    import { APIs } from "../../data/common";
    import { showFailureToast } from "../../data/toast";
    import InfoText from "../type/InfoText.svelte";

    interface Props {
        ondone: () => void;
    }
    let { ondone }: Props = $props();

    let selectedRating: number | null = $state(null);
    let comments = $state("");
    let loading = $state(false);
    let done = $state(false);

    let onSubmit = $derived(async () => {
        if (selectedRating === null) return;

        loading = true;
        try {
            await APIs.feedback.feedbackCreate({
                comment: comments || null,
                score: selectedRating,
            });

            done = true;
        } catch (e) {
            showFailureToast(e);
        }
    });
</script>

<div
    class="fixed bottom-4 right-4 z-10 bg-white dark:bg-slate-800 border border-gray-300 dark:border-gray-700 rounded-xl shadow-lg p-8"
>
    {#if !done}
        <SectionHeading>How are you finding Palform?</SectionHeading>

        {#if selectedRating === null}
            <div class="flex justify-between mt-4">
                {#each [1, 2, 3, 4, 5] as rating}
                    <Button
                        color="light"
                        onclick={() => (selectedRating = rating)}
                        disabled={loading}
                    >
                        {rating}
                    </Button>
                {/each}
            </div>
            <div
                class="flex justify-between mt-1 text-gray-400 dark:text-gray-500"
            >
                <p>Awful</p>
                <p class="text-right">Perfect</p>
            </div>
        {:else}
            <Label class="mt-4">
                Comments (optional)
                <Textarea
                    class="mt-1 w-full"
                    bind:value={comments}
                    disabled={loading}
                />
            </Label>

            <LoadingButton
                disabled={loading}
                {loading}
                buttonClass="mt-4 w-full"
                onclick={onSubmit}
            >
                Submit
            </LoadingButton>
        {/if}
    {:else}
        <SectionHeading>Thanks!</SectionHeading>
        <InfoText class="mt-2 max-w-96" lighter>
            We've anonymously noted your feedback. If you'd like us to respond,
            please email hey@palform.app.
        </InfoText>
        <Button color="light" onclick={() => ondone()} class="mt-4">
            Close
        </Button>
    {/if}
</div>

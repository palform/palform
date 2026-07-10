<script lang="ts">
    import { DateTime } from "luxon";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { parseServerTime } from "../../data/util/time";
    import { Button, Modal, P } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowUpRightFromSquare } from "@fortawesome/free-solid-svg-icons";

    const orgCtx = getOrgContext();

    let isEligible = $derived(() => {
        const orgCreated = parseServerTime($orgCtx.org.created_at);
        const formCount = $orgCtx.forms.length;
        const feedbackPromptComplete =
            localStorage.getItem("feedback-prompt-complete") === "true";
        return (
            orgCreated < DateTime.now().minus({ days: 1 }) &&
            formCount >= 1 &&
            !feedbackPromptComplete
        );
    });

    let showModal = $state(false);
    $effect(() => {
        if (!isEligible) return;

        const timeout = setTimeout(() => {
            showModal = true;
            localStorage.setItem("feedback-prompt-complete", "true");
        }, 60 * 1_000);

        return () => {
            clearTimeout(timeout);
        };
    });
</script>

<Modal bind:open={showModal} title="Help improve Palform">
    <P>
        We'd be grateful if you could spend <strong>2 minutes</strong> to answer some
        anonymous questions about Palform. This would help us improve our service
        for you :)
    </P>

    <P size="xs">Don't worry: we won't show this request again.</P>

    {#snippet footer()}
        <Button href="https://pf.palform.app/feedback" target="_blank">
            <FontAwesomeIcon icon={faArrowUpRightFromSquare} class="me-2" />
            Continue
        </Button>
        <Button color="alternative" onclick={() => (showModal = false)}>
            Close
        </Button>
    {/snippet}
</Modal>

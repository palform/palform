<script lang="ts">
    import { onMount } from "svelte";
    import {
        hiddenQuestionValue,
        setQuestionValue,
        type QuestionFillProps,
    } from "../../../data/contexts/fill";
    import type { ConfigHidden } from "@palform/palform-typescript-openapi";

    interface Props extends QuestionFillProps<ConfigHidden> {}

    let { id, config, onchange }: Props = $props();

    onMount(() => {
        setQuestionValue(id, hiddenQuestionValue(config.hidden.parameter_name));
        onchange();
    });
</script>

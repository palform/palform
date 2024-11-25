<script lang="ts">
    import type { APIQuestionConfigurationOneOf10 } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher, onMount } from "svelte";
    import {
        hiddenQuestionValue,
        setQuestionValue,
    } from "../../../data/contexts/fill";

    export let id: string;
    export let config: APIQuestionConfigurationOneOf10;

    const dispatch = createEventDispatcher<{ change: undefined }>();

    onMount(() => {
        setQuestionValue(id, hiddenQuestionValue(config.hidden.parameter_name));
        dispatch("change");
    });
</script>

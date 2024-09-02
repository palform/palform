<script lang="ts">
    import type { APIQuestionConfigurationOneOf10 } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher, onMount } from "svelte";
    import { setQuestionValue } from "../../../data/contexts/fill";

    export let id: string;
    export let config: APIQuestionConfigurationOneOf10;

    const dispatch = createEventDispatcher<{ change: undefined }>();

    onMount(() => {
        const params = new URLSearchParams(window.location.search);
        const paramValue = params.get(config.hidden.parameter_name);
        setQuestionValue(id, {
            Hidden: {
                value: paramValue ?? "",
            },
        });
        dispatch("change");
    });
</script>

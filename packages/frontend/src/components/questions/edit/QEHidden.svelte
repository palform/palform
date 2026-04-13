<script lang="ts">
    import type { ConfigHidden } from "@palform/palform-typescript-openapi";
    import { Helper, Input, Label } from "flowbite-svelte";
    import { getFormEditorCtx } from "../../../data/contexts/formEditor";

    interface Props {
        config: ConfigHidden;
    }

    let { config = $bindable() }: Props = $props();
    const ctx = getFormEditorCtx();
</script>

<Label>
    Query parameter name
    <Input
        class="mt-2"
        bind:value={config.hidden.parameter_name}
        disabled={$ctx.loading}
    />
    {#if config.hidden.parameter_name.length > 0}
        <Helper class="mt-2">
            We'll take the value from the parameter <code
                >?{config.hidden.parameter_name}=</code
            > in the URL when the form is being filled out.
        </Helper>
    {/if}
</Label>

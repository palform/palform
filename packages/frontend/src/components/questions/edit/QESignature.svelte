<script lang="ts">
    import type { ConfigSignature } from "@palform/palform-typescript-openapi";
    import { Alert, Toggle } from "flowbite-svelte";
    import { getFormEditorCtx } from "../../../data/contexts/formEditor";

    interface Props {
        config: ConfigSignature;
    }

    let { config = $bindable() }: Props = $props();
    const ctx = getFormEditorCtx();
</script>

{#if !config.signature.allow_freeform && !config.signature.allow_initial && !config.signature.allow_full_name}
    <Alert class="mb-4">Please select at least one signature method</Alert>
{/if}

<Toggle bind:checked={config.signature.allow_freeform} disabled={$ctx.loading}>
    Allow freeform hand-drawn signatures
</Toggle>

<Toggle
    class="mt-4"
    bind:checked={config.signature.allow_initial}
    disabled={$ctx.loading}
>
    Allow initials as signature
</Toggle>

<Toggle
    class="mt-4"
    bind:checked={config.signature.allow_full_name}
    disabled={$ctx.loading}
>
    Allow full name as signature
</Toggle>

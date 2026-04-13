<script lang="ts">
    import { Router } from "sv-router";
    import ImperativeToastManager from "./components/toasts/ImperativeToastManager.svelte";
</script>

<div class={`min-h-full h-full`}>
    <Router />
    <ImperativeToastManager />
</div>

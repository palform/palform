<script lang="ts">
    import { Router } from "sv-router";
    import ImperativeToastManager from "./components/toasts/ImperativeToastManager.svelte";
    import DarkModeWatcher from "./components/darkMode/DarkModeWatcher.svelte";
</script>

<div class={`min-h-full h-full`}>
    <Router />
    <ImperativeToastManager />
    <DarkModeWatcher />
</div>

<script lang="ts">
    export let disabled = false;
</script>

<button
    class="text-xs text-gray-600 hover:underline cursor-pointer"
    type="button"
    on:click
    {disabled}
>
    Clear
</button>

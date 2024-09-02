<script lang="ts">
    export let disabled: boolean = false;
</script>

<button
    class="text-primary-700 disabled:text-gray-500 hover:underline"
    {disabled}
    on:click
>
    <slot />
</button>

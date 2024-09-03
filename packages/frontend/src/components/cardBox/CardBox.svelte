<script lang="ts">
    import GenericBrandBox from "../teams/brandings/GenericBrandBox.svelte";

    export let backgroundColor: string = "";
    export let errorState = false;
    export let fadeIn = false;
    export let fadeOut = false;
</script>

<GenericBrandBox
    class={`${errorState ? "border-red-300 dark:border-red-700 shadow-red-200 dark:shadow-red-600" : "border-primary-200/60 dark:border-primary-800/60 shadow-xl shadow-primary-300/10 dark:shadow-primary-700/10"} ${$$props.class ?? ""}`}
    {backgroundColor}
    {fadeIn}
    {fadeOut}
>
    <slot />
</GenericBrandBox>

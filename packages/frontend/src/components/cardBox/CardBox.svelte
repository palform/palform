<script lang="ts">
    import GenericBrandBox from "../teams/brandings/GenericBrandBox.svelte";

    export let backgroundColor: string = "";
    export let errorState = false;
    export let fadeIn = false;
    export let fadeOut = false;
</script>

<GenericBrandBox
    class={`${errorState ? "border-red-300 dark:border-red-700 shadow-red-200 dark:shadow-red-600" : "border-primary-300 dark:border-primary-700 shadow-primary-200 dark:shadow-primary-600"} ${$$props.class ?? ""}`}
    {backgroundColor}
    {fadeIn}
    {fadeOut}
>
    <slot />
</GenericBrandBox>

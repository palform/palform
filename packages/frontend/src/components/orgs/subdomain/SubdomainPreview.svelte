<script lang="ts">
    interface Props {
        subdomain: string;
        [key: string]: any
    }

    let { ...props }: Props = $props();
</script>

<span class={`block text-3xl font-display ${props.class}`}>
    <span class="font-medium">{props.subdomain}</span><span class="font-light"
        >.palform.app/</span
    ><span class="font-light text-gray-400 dark:text-gray-500">my-link</span>
</span>

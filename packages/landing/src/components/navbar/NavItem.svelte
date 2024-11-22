<script lang="ts">
    export let href: string | undefined = undefined;
    export let subItems:
        | { title: string; description: string; href: string }[]
        | undefined = undefined;

    let showMenu = false;
    $: onHover = () => {
        showMenu = true;
    };

    $: console.log(showMenu);
</script>

<li>
    <a {href} on:mouseover={onHover}>
        <slot />
    </a>
</li>

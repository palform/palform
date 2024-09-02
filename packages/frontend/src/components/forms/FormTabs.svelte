<script lang="ts">
    import { Tabs } from "flowbite-svelte";
    import RoutedTabItem from "../tabs/RoutedTabItem.svelte";
</script>

<Tabs contentClass="mt-4">
    <RoutedTabItem title="Overview" path="" />
    <RoutedTabItem title="Responses" path="responses" />
    <RoutedTabItem title="Edit" path="edit" />
    <RoutedTabItem title="Share" path="tokens" />
    <RoutedTabItem title="Export" path="export" />
    <RoutedTabItem title="Settings" path="settings" />
</Tabs>

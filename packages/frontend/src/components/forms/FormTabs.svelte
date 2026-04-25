<script lang="ts">
    import { Tabs } from "flowbite-svelte";
    import RoutedTabItem from "../tabs/RoutedTabItem.svelte";
</script>

<Tabs classes={{ content: "p-0 h-0 m-0" }}>
    <RoutedTabItem title="Overview" path="overview" />
    <RoutedTabItem title="Responses" path="responses" />
    <RoutedTabItem title="Edit" path="edit" />
    <RoutedTabItem title="Publish" path="tokens" />
    <RoutedTabItem title="Export" path="export" />
    <RoutedTabItem title="Settings" path="settings" />
</Tabs>

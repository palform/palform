<script lang="ts">
    import { Button } from "flowbite-svelte";
    import ErrorMsg from "../../components/ErrorMsg.svelte";
    import OrganisationCard from "../../components/orgs/OrganisationCard.svelte";
    import OrganisationCardLoading from "../../components/orgs/OrganisationCardLoading.svelte";
    import { APIs } from "../../data/common";
    import Main from "../../layouts/Main.svelte";
    import CardBox from "../../components/cardBox/CardBox.svelte";
    import CardBoxTitle from "../../components/cardBox/CardBoxTitle.svelte";
    import { navigateEvent } from "@paltiverse/palform-frontend-common";
</script>

<Main title="My organisations">
    <div class="grid sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 mt-4 gap-4">
        {#await APIs.orgs().then((a) => a.orgsList())}
            {#each [1, 2, 3] as _}
                <OrganisationCardLoading />
            {/each}
        {:then resp}
            {#each resp.data as org}
                <OrganisationCard {org} />
            {/each}
            <CardBox class="bg-primary-50 dark:bg-primary-900">
                <CardBoxTitle class="font-medium">
                    Create a new organisation
                </CardBoxTitle>
                <Button class="mt-2" href="/orgs/new" on:click={navigateEvent}>
                    Get started
                </Button>
            </CardBox>
        {:catch e}
            <ErrorMsg {e} />
        {/await}
    </div>
</Main>

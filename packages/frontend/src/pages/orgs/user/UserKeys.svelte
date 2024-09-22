<script lang="ts">
    import {
        Alert,
        Button,
        ButtonGroup,
        Spinner,
        Table,
        TableBody,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faFileImport, faPlus } from "@fortawesome/free-solid-svg-icons";
    import { APIs } from "../../../data/common";
    import type { APIUserKey } from "@paltiverse/palform-typescript-openapi";
    import KeyTableRow from "../../../components/keys/KeyTableRow.svelte";
    import InfoText from "../../../components/type/InfoText.svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import CardGrid from "../../../components/CardGrid.svelte";
    import InductionStepCard from "../../../components/induction/InductionStepCard.svelte";
    import TableContainer from "../../../components/tables/TableContainer.svelte";
    import { isEntitled } from "../../../data/billing/entitlement";
    import MissingEntitlementTooltip from "../../../components/billing/entitlement/MissingEntitlementTooltip.svelte";
    import { navigateEvent } from "@paltiverse/palform-frontend-common";

    export let orgId: string;
    const orgCtx = getOrgContext();
    const importEntitled = isEntitled("import_keys");

    let keys: APIUserKey[] = [];
    let keysLoading = true;
    APIs.keys()
        .then((a) => a.keysList(orgId))
        .then((resp) => {
            keys = resp.data;
            keysLoading = false;
        });

    const onKeyDelete = (id: string) => {
        keys = keys.filter((e) => e.id !== id);
    };
</script>

{#if keysLoading}
    <div class="text-center">
        <Spinner size={14} />
    </div>
{:else}
    <ButtonGroup>
        <Button
            href={`/orgs/${orgId}/user/keys/register`}
            on:click={navigateEvent}
            color="primary"
        >
            <FontAwesomeIcon icon={faPlus} class="me-2" />
            Register new
        </Button>

        <Button
            outline
            color="primary"
            href={`/orgs/${orgId}/user/keys/import`}
            on:click={navigateEvent}
            disabled={!$importEntitled}
        >
            <FontAwesomeIcon icon={faFileImport} class="me-2" />
            Import
        </Button>
        <MissingEntitlementTooltip key="import_keys" />
    </ButtonGroup>

    {#if keys.length === 0}
        <CardGrid class="mt-6">
            <InductionStepCard title="Keys secure your responses">
                Every response to your form will be encrypted with your key.
                This means nobody outside your organisation can see it — not
                even us!
            </InductionStepCard>
            <InductionStepCard title="Stored in your browser">
                That's right — your plain key is super secret so we keep it in
                your browser and never send it to our server (unless you make a
                backup).
            </InductionStepCard>
            <InductionStepCard title="Backups">
                Losing your form responses would be really sad, so you can back
                up your keys to our server, with a secret recovery phrase to
                keep them secure.
            </InductionStepCard>
        </CardGrid>
    {:else}
        <InfoText class="mt-4">
            Shown below are your keys within {$orgCtx.org.display_name}. Keys
            belonging to other members of your organisation are not shown.
        </InfoText>

        <Alert color="gray" border class="mt-4">
            <h3 class="text-lg">Legend</h3>
            <p>
                <span class="text-green-600">Active</span>: New responses are
                encrypted with these keys.
            </p>
            <p>
                <span class="text-red-600 line-through">Expired</span>: Existing
                responses can be decrypted with these keys, but new responses
                won't be encrypted with them.
            </p>
        </Alert>

        <TableContainer class="mt-4">
            <Table divClass="">
                <TableHead>
                    <TableHeadCell>Fingerprint</TableHeadCell>
                    <TableHeadCell>Present in your browser</TableHeadCell>
                    <TableHeadCell>Backup exists</TableHeadCell>
                    <TableHeadCell>Created</TableHeadCell>
                    <TableHeadCell>Expires</TableHeadCell>
                    <TableHeadCell>
                        <span class="sr-only">Actions</span>
                    </TableHeadCell>
                </TableHead>
                <TableBody>
                    {#each keys as key (key.id)}
                        <KeyTableRow
                            {key}
                            on:deleted={() => onKeyDelete(key.id)}
                        />
                    {/each}
                </TableBody>
            </Table>
        </TableContainer>
    {/if}
{/if}

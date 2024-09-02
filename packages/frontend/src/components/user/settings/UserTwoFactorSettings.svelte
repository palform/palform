<script lang="ts">
    import type { APIAdminUserSecondAuthenticationFactor } from "@paltiverse/palform-typescript-openapi";
    import SectionHeading from "../../type/SectionHeading.svelte";
    import { APIs } from "../../../data/common";
    import SkeletonPrimitive from "../../SkeletonPrimitive.svelte";
    import { showFailureToast } from "../../../data/toast";
    import TableContainer from "../../tables/TableContainer.svelte";
    import {
        Table,
        TableBody,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import UserTwoFactorRow from "./UserTwoFactorRow.svelte";
    import UserTwoFactorEnroll from "./UserTwoFactorEnroll.svelte";

    let methods: APIAdminUserSecondAuthenticationFactor[] | undefined;
    let loading = true;
    APIs.secondFactors()
        .then((a) => a.userSecondFactorsList())
        .then((resp) => {
            methods = resp.data;
            loading = false;
        })
        .catch(showFailureToast);

    $: onEnroll = (e: CustomEvent<APIAdminUserSecondAuthenticationFactor>) => {
        if (!methods) return;
        methods = [e.detail, ...methods];
    };

    $: onDelete = (id: string) => {
        if (!methods) return;
        methods = methods.filter((e) => e.id !== id);
    };
</script>

<SectionHeading class="mb-4">Two-factor authentication</SectionHeading>

{#if loading}
    <SkeletonPrimitive height="40px" />
    <SkeletonPrimitive height="30px" className="mt-1" />
    <SkeletonPrimitive height="30px" className="mt-1" />
    <SkeletonPrimitive height="30px" className="mt-1" />
{/if}

{#if methods}
    <UserTwoFactorEnroll on:enroll={onEnroll} />
    {#if methods.length > 0}
        <TableContainer class="mt-4">
            <Table divClass="">
                <TableHead theadClass="sr-only">
                    <TableHeadCell>Nickname</TableHeadCell>
                    <TableHeadCell>Created</TableHeadCell>
                    <TableHeadCell>Actions</TableHeadCell>
                </TableHead>
                <TableBody>
                    {#each methods as method (method.id)}
                        <UserTwoFactorRow
                            {method}
                            on:delete={() => onDelete(method.id)}
                        />
                    {/each}
                </TableBody>
            </Table>
        </TableContainer>
    {/if}
{/if}

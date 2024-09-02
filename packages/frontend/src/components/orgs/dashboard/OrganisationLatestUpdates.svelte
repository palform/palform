<script lang="ts">
    import { APIs } from "../../../data/common";
    import {
        getLastOrgView,
        getOrgContext,
        markOrgViewNow,
    } from "../../../data/contexts/orgLayout";
    import type { APISubmissionCountPerForm } from "@paltiverse/palform-typescript-openapi";
    import BigStat from "../../type/BigStat.svelte";
    import CardBox from "../../cardBox/CardBox.svelte";
    import BigStatCaption from "../../type/BigStatCaption.svelte";
    import { Table, TableBody } from "flowbite-svelte";
    import LatestUpdateRow from "./LatestUpdateRow.svelte";
    import TableContainer from "../../tables/TableContainer.svelte";

    const orgCtx = getOrgContext();
    let newSubmissions: APISubmissionCountPerForm[] = [];
    let loading = true;
    getLastOrgView($orgCtx.org.id)
        .then(async (lv) => {
            if (!lv) {
                newSubmissions = [];
                return;
            }

            const api = await APIs.submissions();
            return await api.submissionsNumSince($orgCtx.org.id, {
                since: lv.toISO() ?? "",
            });
        })
        .then(async (resp) => {
            await markOrgViewNow($orgCtx.org.id);

            if (!resp) return;
            newSubmissions = resp.data;
            loading = false;
        });

    $: totalNewSubmissions = newSubmissions.reduce(
        (t, c) => t + c.new_submission_count,
        0
    );
</script>

<CardBox class="mt-4 inline-block min-w-96">
    <BigStat>{totalNewSubmissions}</BigStat>
    <BigStatCaption>
        new response{totalNewSubmissions === 1 ? "" : "s"}
    </BigStatCaption>

    {#if newSubmissions.length > 0}
        <TableContainer class="mt-3">
            <Table divClass="">
                <TableBody>
                    {#each newSubmissions as submissionGroup (submissionGroup.form_id)}
                        <LatestUpdateRow data={submissionGroup} />
                    {/each}
                </TableBody>
            </Table>
        </TableContainer>
    {/if}
</CardBox>

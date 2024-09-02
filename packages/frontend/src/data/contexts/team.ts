import type {
    APIFormBranding,
    APIOrganisationTeam,
    APIOrganisationTeamMember,
} from "@paltiverse/palform-typescript-openapi";
import { getContext, setContext } from "svelte";
import type { Writable } from "svelte/store";

export interface TeamContext {
    team: APIOrganisationTeam;
    members: APIOrganisationTeamMember[];
    brandings: APIFormBranding[];
}

export const setTeamCtx = (ctx: Writable<TeamContext>) => {
    setContext("current_team_manage", ctx);
};
export const getTeamCtx = () =>
    getContext<Writable<TeamContext>>("current_team_manage");

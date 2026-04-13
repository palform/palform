import {
    type APIQuestionGroupStepStrategyJumpCaseConditionMatcher,
    type APIQuestionGroupStepStrategyJumpCaseConditionList,
    type MatcherText,
    type MatcherChoice,
    type MatcherScale,
    type MatcherPhoneNumber,
    type MatcherAddress,
    type MatcherChoiceMatrix,
    type MatcherDateTime,
    type MatcherHidden,
} from "@palform/palform-typescript-openapi";
import { comparisonSymbol } from "./directionOperator";
import { DateTime } from "luxon";
import { labelForQuestionDate, timeZoneSummary } from "./time";

export function extractConditionList(
    conditionList: APIQuestionGroupStepStrategyJumpCaseConditionList
) {
    if ("Or" in conditionList) {
        return conditionList.Or;
    }
    return conditionList.And;
}

export function matcherLabel(
    matcher: APIQuestionGroupStepStrategyJumpCaseConditionMatcher
) {
    if (conditionMatcherIsText(matcher)) {
        return `${matcher.Text.contains ? "contains" : "is"} "${
            matcher.Text.value
        }"${matcher.Text.case_sensitive ? ", case sensitive" : ""}`;
    }
    if (conditionMatcherIsChoice(matcher)) {
        return `${matcher.Choice.contains_any ? "contains any of" : "is"} ${matcher.Choice.options.join(", ")}`;
    }
    if (conditionMatcherIsScale(matcher)) {
        const operatorSymbol = comparisonSymbol(matcher.Scale.direction);
        return `${operatorSymbol} ${matcher.Scale.value}`;
    }
    if (conditionMatcherIsPhoneNumber(matcher)) {
        return `calling code is ${matcher.PhoneNumber.calling_code}`;
    }
    if (conditionMatcherIsAddress(matcher)) {
        let text = "";
        if (matcher.Address.near) {
            if (matcher.Address.near_radius_km) {
                text += `within ${matcher.Address.near_radius_km}km of `;
            } else {
                text += "near ";
            }
            text += `(${matcher.Address.near.lat}, ${matcher.Address.near.lng})`;

            if (matcher.Address.in_country) {
                text += " and ";
            }
        }

        if (matcher.Address.in_country) {
            text += `in country ${matcher.Address.in_country}`;
        }
        return text;
    }
    if (conditionMatcherIsChoiceMatrix(matcher)) {
        return `${matcher.ChoiceMatrix.row} is/contains ${matcher.ChoiceMatrix.column}`;
    }
    if (conditionMatcherIsDateTime(matcher)) {
        let d = DateTime.fromISO(matcher.DateTime.value);

        const label = labelForQuestionDate(
            {
                collect_date: matcher.DateTime.match_date,
                collect_time: matcher.DateTime.match_time,
            },
            d
        );
        return `${comparisonSymbol(matcher.DateTime.direction)} ${label} (${timeZoneSummary(d, true)})`;
    }
    if (conditionMatcherIsHidden(matcher)) {
        return `is ${matcher.Hidden.value}`;
    }

    return "";
}

function conditionMatcherIs<
    T extends APIQuestionGroupStepStrategyJumpCaseConditionMatcher,
>(
    label: string
): (
    strategy: APIQuestionGroupStepStrategyJumpCaseConditionMatcher
) => strategy is T {
    return (
        strategy: APIQuestionGroupStepStrategyJumpCaseConditionMatcher
    ): strategy is T => Object.keys(strategy)[0] === label;
}
export const conditionMatcherIsText = conditionMatcherIs<MatcherText>("Text");
export const conditionMatcherIsChoice =
    conditionMatcherIs<MatcherChoice>("Choice");
export const conditionMatcherIsScale =
    conditionMatcherIs<MatcherScale>("Scale");
export const conditionMatcherIsPhoneNumber =
    conditionMatcherIs<MatcherPhoneNumber>("PhoneNumber");
export const conditionMatcherIsAddress =
    conditionMatcherIs<MatcherAddress>("Address");
export const conditionMatcherIsChoiceMatrix =
    conditionMatcherIs<MatcherChoiceMatrix>("ChoiceMatrix");
export const conditionMatcherIsDateTime =
    conditionMatcherIs<MatcherDateTime>("DateTime");
export const conditionMatcherIsHidden =
    conditionMatcherIs<MatcherHidden>("Hidden");

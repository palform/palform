import type { APIQuestionConfigurationOneOf9DateTime } from "@paltiverse/palform-typescript-openapi";
import { DateTime } from "luxon";

export function parseServerTime(timeString: string, utc = false) {
    const dt = DateTime.fromISO(timeString, { zone: "utc" });
    if (utc) {
        return dt.toUTC();
    } else {
        return dt.toLocal();
    }
}

export function isDateOnlyEqual(d1: DateTime, d2: DateTime) {
    return d1.day === d2.day && d1.month === d2.month && d1.year === d2.year;
}

export function labelForQuestionDate(
    question: APIQuestionConfigurationOneOf9DateTime,
    date: DateTime | string | null | undefined
) {
    if (!date) return "";

    if (typeof date === "string") {
        date = DateTime.fromISO(date);
    }

    return date.toLocaleString(
        question.collect_date && question.collect_time
            ? DateTime.DATETIME_MED
            : question.collect_date
              ? DateTime.DATE_MED
              : DateTime.TIME_24_SIMPLE
    );
}

export function timeZoneSummary(value: DateTime, comma = false) {
    return `${value.offsetNameShort}${comma ? ", " : " ("}${
        value.offset < 0 ? "-" : "+"
    }${value.offset / 60}h${comma ? "" : ")"}`;
}

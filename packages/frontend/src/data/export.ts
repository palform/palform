import { export_submissions_js } from "@palform/palform-client-common";
import {
    submissionIsSuccess,
    type DecryptedSubmissionSuccess,
} from "./crypto/results";
import downloadFile from "./util/downloadFile";
import type { FormAdminContext } from "./contexts/formAdmin";

export interface ExportSubmissionsConfig {
    use_question_ids: boolean;
    use_group_ids: boolean;
    format: "JSON" | "CSV";
}

export const exportFormats: {
    name: string;
    value: ExportSubmissionsConfig["format"];
}[] = [
    {
        name: "CSV",
        value: "CSV",
    },
    {
        name: "JSON",
        value: "JSON",
    },
];

export function exportFormSubmissions(
    ctx: FormAdminContext,
    config: ExportSubmissionsConfig
) {
    const submissions = ctx.submissions
        .filter((e) => submissionIsSuccess(e))
        .map((_e) => {
            const e = _e as DecryptedSubmissionSuccess;
            return {
                form_id: ctx.formId,
                groups_completed: ctx.groups.map((e) => e.id),
                questions: e.questions,
            };
        });

    const resp = export_submissions_js(
        ctx.groups,
        submissions,
        ctx.questions,
        config
    );

    let extension = "";
    switch (config.format) {
        case "CSV":
            extension = "csv";
            break;
        case "JSON":
            extension = "json";
    }

    downloadFile(`form_export.${extension}`, resp);
}

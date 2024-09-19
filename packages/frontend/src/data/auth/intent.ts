export function saveIntentTemplateIfExists() {
    const templateId = new URLSearchParams(window.location.search).get(
        "intentTemplate"
    );
    if (!templateId) return false;
    sessionStorage.setItem("intent_template", templateId);
    return true;
}

export function getIntentTemplate() {
    const templateId = sessionStorage.getItem("intent_template");
    if (templateId) {
        sessionStorage.removeItem("intent_template");
    }
    return templateId;
}

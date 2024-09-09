<script>
    import InfoText from "../../type/InfoText.svelte";
    import SectionHeading from "../../type/SectionHeading.svelte";
</script>

<SectionHeading class="mb-4">Language</SectionHeading>

<InfoText lighter class="mb-2">
    Any text on your form will be shown in the user's browser language, subject
    to support. It's up to you to provide the actual questions in the relevant
    language.
</InfoText>

<InfoText lighter>
    Currently, the following languages are supported:

    <ul class="mt-2 space-y-1 list-disc list-inside">
        <li>English</li>
        <li>German</li>
    </ul>
</InfoText>

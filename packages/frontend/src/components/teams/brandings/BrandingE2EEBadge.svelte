<script>
    import { faLock } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../../data/contexts/brand";
    import { isDarkMode } from "../../../data/util/darkMode";
    import { colorWithLightness } from "../../../data/util/color";
    import { Modal } from "flowbite-svelte";
    import TextButton from "../../TextButton.svelte";
    import { formFillStore } from "../../../data/contexts/fill";

    const brandCtx = getBrandCtx();
    const isDark = isDarkMode();

    $: buttonColor = $brandCtx
        ? isDark
            ? colorWithLightness($brandCtx.primary_color, 20)
            : colorWithLightness($brandCtx.primary_color, 90)
        : undefined;

    $: iconColor = $brandCtx
        ? isDark
            ? colorWithLightness($brandCtx.primary_color, 80)
            : colorWithLightness($brandCtx.primary_color, 30)
        : undefined;

    let showModal = false;
</script>

<div class="fixed bottom-4 right-4">
    <button
        class="px-3 py-2 md:px-4 md:py-3 rounded-lg bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 text-sm flex items-center text-left"
        style:background-color={buttonColor}
        style:color={iconColor}
        style:border-radius={getRoundingAmountForBrand($brandCtx, true)}
        on:click={() => (showModal = true)}
    >
        <span class="me-3">
            <FontAwesomeIcon icon={faLock} size="md" />
        </span>
        <span class="leading-tight"
            >Encrypted<br /><span class="text-xs">by Palform</span></span
        >
    </button>
</div>

<Modal title="Encrypted response" bind:open={showModal} outsideclose>
    <p>
        This form is hosted by <TextButton
            class="inline !text-base"
            href="https://palform.app/?utm_source=encrypt_modal"
            target="_blank">Palform</TextButton
        >.
    </p>

    <p>
        Your response will be <strong>end-to-end encrypted</strong>, so only the
        form's owner (<strong>{$formFillStore?.form.o}</strong>) will be able to
        see any information you submit.
    </p>

    <p>
        If you need any help, please contact {$formFillStore?.form.o} or email Palform
        at <strong>responses@palform.app</strong>.
    </p>
</Modal>

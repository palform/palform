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
    import { t } from "../../../data/contexts/i18n";

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

<div class="fixed bottom-4 right-4 z-20">
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
            >{t("encrypted_badge_1")}<br /><span class="text-xs"
                >{t("encrypted_badge_2")}</span
            ></span
        >
    </button>
</div>

<Modal title={t("encrypted_modal_title")} bind:open={showModal} outsideclose>
    <p>
        {t("encrypted_modal_1")}<TextButton
            class="inline !text-base"
            href="https://palform.app/?utm_source=encrypt_modal"
            target="_blank">Palform</TextButton
        >{t("encrypted_modal_2")}.
    </p>

    <p>
        {t("encrypted_modal_3")}(<strong>{$formFillStore?.form.o}</strong>){t(
            "encrypted_modal_4"
        )}.
    </p>

    <p>
        {t("encrypted_modal_5")}{$formFillStore?.form.o}{t(
            "encrypted_modal_6"
        )}<strong>dpo@palform.app</strong>{t("encrypted_modal_7")}.
    </p>
</Modal>

import { type APIFormBranding } from "@paltiverse/palform-typescript-openapi";
import { getContext, setContext } from "svelte";
import type { Writable } from "svelte/store";

export type BrandContext = Pick<
    APIFormBranding,
    | "google_font"
    | "primary_color"
    | "accent_color"
    | "border_rounding"
    | "spacing"
    | "font_size"
    | "logo_asset_id"
    | "background_image_asset_id"
    | "extra_footer_message"
    | "terms_link"
    | "privacy_link"
    | "include_palform_attribution"
    | "border_intensity"
    | "border_shadow_intensity"
    | "e2ee_badge"
    | "background_color"
    | "background_color_accent"
>;
export const setBrandCtx = (ctx: Writable<BrandContext | undefined>) =>
    setContext("branding", ctx);
export const getBrandCtx = () =>
    getContext<Writable<BrandContext | undefined>>("branding");

export function getRoundingAmountForBrand(brand?: BrandContext, inner = false) {
    if (!brand) return inner ? "0.65rem" : "1rem";
    switch (brand.border_rounding) {
        case "Large":
            return inner ? "1.2rem" : "2rem";
        case "Medium":
            return inner ? "0.65rem" : "1rem";
        case "Small":
            return inner ? "0.3rem" : "0.5rem";
        case "None":
            return "0";
    }
}

export function getPaddingAmountForBrand(brand?: BrandContext, half = false) {
    if (!brand) return half ? "0.5rem" : "1rem";
    switch (brand.spacing) {
        case "Comfy":
            return half ? "0.7rem" : "1.3rem";
        case "Normal":
            return half ? "0.5rem" : "1rem";
        case "Tight":
            return half ? "0.3rem" : "0.7rem";
    }
}

export function getBaseREMFontSizeForBrand(brand?: BrandContext): number {
    if (!brand) return 1;
    switch (brand.font_size) {
        case "Tiny":
            return 0.8;
        case "Small":
            return 0.9;
        case "Regular":
            return 1;
        case "Large":
            return 1.2;
        case "VeryLarge":
            return 1.4;
    }
}

export function getLightnessForBrandBorder(
    brand?: BrandContext
): number | undefined {
    if (!brand) return 75;
    switch (brand.border_intensity) {
        case "High":
            return 50;
        case "Medium":
            return 75;
        case "Low":
            return 90;
        case "Off":
            return undefined;
    }
}

export function getShadowAlphaForBrandBorder(brand?: BrandContext): number {
    if (!brand) return 0.4;
    switch (brand.border_shadow_intensity) {
        case "High":
            return 0.2;
        case "Medium":
            return 0.1;
        case "Low":
            return 0.03;
        case "Off":
            return 0;
    }
}

export function getBrandIsNonNeutralBackground(brand?: BrandContext) {
    return (
        brand !== undefined &&
        (brand.background_image_asset_id != null || brand.background_color != null)
    );
}

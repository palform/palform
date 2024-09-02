import type { APIGenericAddress } from "@paltiverse/palform-client-js-extra-types/APIGenericAddress";
import { APIs } from "../common";
import {
    AddressAutofillCore,
    SearchSession,
    type AddressAutofillRetrieveResponse,
    type AddressAutofillSuggestion,
} from "@mapbox/search-js-core";
import type { APIGenericLocation } from "@paltiverse/palform-client-js-extra-types/APIGenericLocation";

const mapboxAPIKey = import.meta.env.VITE_MAPBOX_API_KEY as string;

export type MapboxAPIType = ReturnType<typeof getMapboxAPI>;
export function getMapboxAPI() {
    const client = new AddressAutofillCore({
        accessToken: mapboxAPIKey,
    });
    return new SearchSession(client);
}

export function parseAddressComponents(suggestion: AddressAutofillSuggestion) {
    const address: APIGenericAddress = {
        line1: suggestion.address_line1 ?? null,
        line2: suggestion.address_line2 ?? null,
        postal_code: suggestion.postcode ?? null,
        locality: suggestion.address_level2 ?? null,
        iso_3166_alpha_1_code: suggestion.country_code?.toUpperCase() ?? null,
    };

    return address;
}

export function parseResponseGeometry(
    resp: AddressAutofillRetrieveResponse
): APIGenericLocation | undefined {
    if (resp.features.length === 0) return undefined;
    const f = resp.features[0];
    if (f.geometry.type !== "Point") {
        return undefined;
    }

    return {
        lat: f.geometry.coordinates[1],
        lng: f.geometry.coordinates[0],
    };
}

export async function getCountryList() {
    const resp = await APIs.countries.countriesListNames();
    return resp.data.map((d) => ({ name: d.name, value: d.iso_code }));
}

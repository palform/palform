<script lang="ts">
    import { v4 as uuid } from "uuid";
    import Map from "ol/Map";
    import View from "ol/View";
    import TileLayer from "ol/layer/Tile";
    import HeatmapLayer from "ol/layer/Heatmap";
    import { fromLonLat } from "ol/proj";
    import { OSM } from "ol/source";
    import VectorSource from "ol/source/Vector";
    import Feature from "ol/Feature";
    import { Point } from "ol/geom";
    import type Layer from "ol/layer/Layer";
    import { Vector as VectorLayer } from "ol/layer";
    import Style from "ol/style/Style";
    import Icon from "ol/style/Icon";

    export let heatmapPoints: [number, number][] | undefined = undefined;
    export let pinPoints: [number, number][] | undefined = undefined;
    export let centerOn: [number, number] | undefined = undefined;

    const mapId = uuid();
    let map: Map | null = null;
    $: setupMap = (_: HTMLDivElement) => {
        const layers: Layer[] = [new TileLayer({ source: new OSM() })];
        if (heatmapPoints) {
            layers.push(
                new HeatmapLayer({
                    radius: 14,
                    blur: 20,
                    source: new VectorSource({
                        features: heatmapPoints.map(
                            ([lat, lng]) =>
                                new Feature(new Point(fromLonLat([lng, lat])))
                        ),
                    }),
                })
            );
        }

        if (pinPoints) {
            const pointStyle = new Style({
                image: new Icon({
                    anchor: [0.5, 1],
                    src: "/icons/map_pin.svg",
                    color: "#ffffff",
                }),
            });

            layers.push(
                new VectorLayer({
                    source: new VectorSource({
                        features: pinPoints.map(([lat, lng]) => {
                            const f = new Feature(
                                new Point(fromLonLat([lng, lat]))
                            );
                            f.setStyle(pointStyle);
                            return f;
                        }),
                    }),
                })
            );
        }

        let view: View;
        if (centerOn) {
            view = new View({
                center: fromLonLat([centerOn[1], centerOn[0]]),
                zoom: 15,
            });
        } else {
            view = new View({
                center: fromLonLat([0, 0]),
                zoom: 0,
            });
        }

        map = new Map({
            target: mapId,
            view,
            layers,
        });

        return {
            destroy() {
                if (map) {
                    map.setTarget(undefined);
                    map = null;
                }
            },
        };
    };
</script>

<div id={mapId} use:setupMap class="h-96" />

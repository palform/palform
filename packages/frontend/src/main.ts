import App from "./App.svelte";
import "./main.css";
import "ol/ol.css";
import "@fontsource-variable/archivo";
import "@fontsource-variable/archivo/wdth-italic.css";

const app = new App({
    target: document.getElementById("app")!,
});

export default app;

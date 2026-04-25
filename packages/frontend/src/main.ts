import "./router";
import App from "./App.svelte";
import "./main.css";
import "ol/ol.css";
import "@fontsource-variable/archivo";
import { mount } from "svelte";

const app = mount(App, {
    target: document.getElementById("app")!,
});

export default app;

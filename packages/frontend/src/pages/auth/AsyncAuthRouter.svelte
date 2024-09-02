<script>
    import LazyComponent from "../../components/lazy/LazyComponent.svelte";
    const AuthRouter = import("./AuthRouter.svelte");
</script>

<LazyComponent component={AuthRouter} props={{}} />

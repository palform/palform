<script>
    import { Route, Router } from "svelte-routing";
    import Login from "./Login.svelte";
    import SecondFactor from "./SecondFactor.svelte";
    import Callback from "./Callback.svelte";
    import Signup from "./Signup.svelte";
    import VerifyEmailCallback from "./VerifyEmailCallback.svelte";
    import StartPasswordReset from "./StartPasswordReset.svelte";
    import ResetPassword from "./ResetPassword.svelte";
</script>

<Router>
    <Route path="/login" component={Login} />
    <Route path="/login/tfa/:sessionId" component={SecondFactor} />
    <Route path="/callback" component={Callback} />
    <Route path="/signup" component={Signup} />
    <Route
        path="/verify/:verificationId"
        component={VerifyEmailCallback}
    />
    <Route path="/reset/password" component={StartPasswordReset} />
    <Route
        path="/reset/password/:verificationId"
        component={ResetPassword}
    />
</Router>

import React from "react";
import ReactDOM from "react-dom";
import "./src/css/page.less";
import {HashRouter as Router, Route, Switch} from "react-router-dom";
import Login from "./src/features/login/login";
import App from "./src/app";
import {AuthenticationWrapper} from "./src/components/AuthenticationWrapper/AuthenticationWrapper";

const renderApp = () =>
    <AuthenticationWrapper>
        <App/>
    </AuthenticationWrapper>;

ReactDOM.render(<Router>
    <Switch>
        <Route exact path='/login' component={Login}/>
        <Route render={renderApp}/>
    </Switch>
</Router>, document.getElementById("root"));

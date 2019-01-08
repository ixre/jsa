import React from "react";
import { Route, Switch} from "react-router-dom";
import Login from "./features/login/login";
import Index from "./features/home";

const profile = React.lazy(()=>import("./features/profile"));

export default class App extends React.Component {
    constructor(props) {
        super(props);
    }

    render() {
        return (
            <React.Suspense fallback={<div>Loading...</div>}>
                <div className="app-container">
                        <Switch>
                            <Route exact path='/' component={Index}/>
                            <Route path='/profile/edit' component={profile.EditProfile}/>
                            <Route path='/login' component={Login}/>
                        </Switch>
                </div>
            </React.Suspense>
        );
    }
}
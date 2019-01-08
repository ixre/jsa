import React from "react";
import {HashRouter as Router, Route, Switch} from "react-router-dom";
import Login from "../login/login";


export class Index extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }
    render() {
        return <div>...</div>;
    }
}


export default class IndexRouters extends React.Component {
    render() {
        return (
            <Router>
                <Switch>
                    <Route exact path='/' component={Index}/>
                    <Route path='/login' component={Login}/>
                </Switch>
            </Router>
        )
    }
}


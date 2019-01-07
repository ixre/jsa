import React from "react";
import {HashRouter as Router, Route, Switch} from "react-router-dom";
import Login from "../login/login";


export default class Main extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }
    render() {
        return <div>...</div>;
    }
}


export class IndexRouters extends React.Component {
    render() {
        return (
            <Router>
                <Switch>
                    <Route exact path='/' component={Main}/>
                    <Route path='/login' component={Login}/>
                </Switch>
            </Router>
        )
    }
}


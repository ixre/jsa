import React from 'react';
import {HashRouter as Router, Route, Switch} from 'react-router-dom';
import Main from './main';
import Login from "./login"

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


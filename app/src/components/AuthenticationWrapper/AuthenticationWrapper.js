import React from "react"

export class AuthenticationWrapper extends React.Component {
    componentWillReceiveProps() {

    }

    render() {
       return <React.Fragment>123{this.props.children}</React.Fragment>
    }
}
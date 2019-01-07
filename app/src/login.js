import React from "react";
import Button from "antd/lib/button"

export default class Login extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }

    render() {
        return <div>
            <Button type="primary">Primary</Button>
            login page
        </div>;
    }
}

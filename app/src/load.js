import React, {Component} from "react";

export class LazyLoad extends Component {
    constructor(props) {
        super(props);
    }
    render() {
        return (
            <div className="show">
                it is lazy load
            </div>
        )
    }
}
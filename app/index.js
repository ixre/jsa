import React from "react";
import ReactDOM from "react-dom";
import "./src/css/page.less";
class App extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            routes:<span></span>
        };
    }
    componentDidMount() {
        import("./src/features/home/main").then(({IndexRouters}) => {
            this.setState({
                routes: <IndexRouters/>
            });
        });
    }

    render() {
        return (
            <div className="app-container">
                {this.state.routes}
            </div>
        );
    }
}

ReactDOM.render(<App/>, document.getElementById("root"));

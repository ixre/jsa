import React from "react";
import ReactDOM from "react-dom";
import "./src/css/page.less";
const Loading = props => <div>{props.text}</div>;
class App extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            child: <Loading text={"click load"}/>,
            routes:<div></div>
        };
    }

    componentDidMount() {
        import("./src/routes").then(({IndexRouters}) => {
            this.setState({
                routes: <IndexRouters/>
            });
        });
    }

    //动态导入
    dynImport() {
        import("./src/load").then(({LazyLoad}) => {
            this.setState({
                child: <LazyLoad/>
            });
        });
    };

    render() {
        return (
            <div>
                <div onClick={this.dynImport.bind(this)}>
                    <button>click me</button>
                    {this.state.child}
                </div>
                {this.state.routes}
            </div>
        );
    }
}

ReactDOM.render(<App/>, document.getElementById("root"));

import React, {Component} from "react";
import ReactDOM from "react-dom";

const Loading = props => <div>{props.text}</div>;
class App extends Component {
    constructor(props) {
        super(props);
        this.state = {child: <Loading text={"click load"}/>};
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
            <div onClick={this.dynImport.bind(this)}>
                <button>click me</button>
                {this.state.child}
            </div>
        );
    }
}
ReactDOM.render(<App/>, document.getElementById("root"));

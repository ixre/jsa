import React from "react";

const IndexRoutes = React.lazy(() => import("./features/home"));

export default class App extends React.Component {
    constructor(props) {
        super(props);
    }

    render() {
        return (
            <React.Suspense fallback={<div>Loading...</div>}>
                <div className="app-container">
                    <IndexRoutes/>
                </div>
            </React.Suspense>
        );
    }
}
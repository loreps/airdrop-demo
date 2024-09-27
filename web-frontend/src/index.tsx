import React from 'react';
import ReactDOM from 'react-dom/client';
import {
    BrowserRouter,
    Route,
    Routes,
    useParams,
    useSearchParams,
} from 'react-router-dom';
import './index.css';
import App from './App';
import GraphQLProvider from './GraphQLProvider';
import reportWebVitals from './reportWebVitals';

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
root.render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path=":chainId" element={<GraphQLApp />} />
      </Routes>
    </BrowserRouter>
  </React.StrictMode>
);

function GraphQLApp() {
    const { chainId } = useParams();
    const [searchParams] = useSearchParams();

    let app = searchParams.get("app");
    let owner = searchParams.get("owner");
    let host = searchParams.get("host");
    let port = searchParams.get("port");

    if (chainId == null) {
        throw Error("The URL is missing the chain ID");
    }
    if (app == null) {
        throw Error("The URL is missing an `app` query parameter with the application ID");
    }
    if (owner == null) {
        throw Error("The URL is missing an `owner` query parameter with the owner ID");
    }
    if (host == null) {
        console.log(
            "WARN: the URL does not have a `host` query paramater with the address of the local " +
            "node service. Assuming it is `localhost`."
        );
        host = "localhost";
    }
    if (port == null) {
        console.log(
            "WARN: the URL does not have a `port` query paramater with the port of the local " +
            "node service. Assuming it is `8080`."
        );
        port = "8080";
    }

    return (
        <GraphQLProvider chainId={chainId} applicationId={app} host={host} port={port}>
            <App />
        </GraphQLProvider>
    );
}

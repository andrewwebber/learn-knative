import {
    opine,
    React,
    ReactDOMServer,
} from "./deps.ts";

import App from "./app.tsx";
import { config } from 'https://deno.land/x/dotenv/mod.ts';

const port = Number(config()['PORT']) || 1993;
const app = opine();
const browserBundlePath = "/browser.js";

const js =
    `import React from "https://dev.jspm.io/react@16.13.1";\nimport ReactDOM from "https://dev.jspm.io/react-dom@16.13.1";\nconst App = ${App};\nReactDOM.hydrate(React.createElement(App), document.body);`;

const html =
    `<html><head><script type="module" src="${browserBundlePath}"></script><style>* { font-family: Helvetica; }</style></head><body>${
    (ReactDOMServer as any).renderToString(<App />)
    }</body></html>`;

app.use(browserBundlePath, (req, res, next) => {
    res.type("application/javascript").send(js);
});

app.use("/", (req, res, next) => {
    res.type("text/html").send(html);
});

app.listen({ port: port });

console.log(`React SSR App listening on port ${port}`);
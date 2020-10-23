const app = require("express")();
const { CloudEvent, Emitter, Protocol, Version } = require("cloudevents");

console.log("Assuming  Broker " + process.env.BROKER);

const emitter = new Emitter({
    url: process.env.BROKER
});

const type = "node.request";
const source = "https://node-service";
const data = {
    id: 99,
    message: "hello from node-service"
};

console.log(data);

const event = new CloudEvent({
    type,
    source,
    data
});

app.get("/", (req, res) => {
    res.status(200).json(process.env);
});

app.post("/", (req, res) => {
    console.log(req);

    emitter
        .send(event, { protocol: 1 })
        .then(response => {
            console.log(response);
            res.status(201).json(response);
        })
        .catch(err => {
            console.log(err);
            res.status(500).json(err);
        });
});

app.listen(3000, () => {
    console.log("node-service app listening on port 3000!");
});

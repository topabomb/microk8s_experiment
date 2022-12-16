import restana from "restana";
const service = restana();
const port = 3000;
service.get("/hi", async (req, res) =>
    res.send({
        success: true,
        message: "hello world!",
        timestamp:Date.now(),
        request: `${req.headers.host}${req.url}`,
    })
);
console.log(`listening on ${port}`);
service.start(port);

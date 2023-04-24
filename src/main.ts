import express from "express";
import cors from "cors";
import bodyParser from "body-parser";
import morgan from "morgan";

import find from "./routes/find";
import cabra from "./routes/cabra";

const api = express();

api.use(cors())
api.use(bodyParser.urlencoded({ extended: true }))
api.use(bodyParser.json())
api.disable('x-powered-by')
api.use(morgan("dev"))

api.use("/find", find);
api.use("/cabra", cabra)

api.use((req, res, next) => {
    res.header('Access-Control-Allow-Origin', '*')
    res.header('Access-Control-Allow-Methods', 'GET,PUT,POST,DELETE')
    res.header('Access-Control-Allow-Headers', 'Content-Type')
    next()
})

api.use((req, res) => {
    res.status(404).json({ error: 'Not Found' })
});

api.listen(3000, () => {
    console.log('Server started on port 3000')
});
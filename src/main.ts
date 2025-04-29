import express from "express";
import cors from "cors";
import bodyParser from "body-parser";
import morgan from "morgan";
import helmet from "helmet"

import find from "./routes/find";
import cabra from "./routes/cabra";
import nutrias from "./routes/nutria";
import big_cats from "./routes/big_cats";
import pandas from "./routes/panda";

const api = express();

api.use(cors())
api.use(bodyParser.urlencoded({ extended: true }))
api.use(bodyParser.json())
api.disable('x-powered-by')
api.use(morgan("dev"))

api.use(helmet({
    crossOriginEmbedderPolicy: false,
}));

api.use("/find", find);
api.use("/cabra", cabra)
api.use("/nutria", nutrias)
api.use("/big-cat", big_cats)
api.use("/panda", pandas)

api.use((_req, res) => {
    res.status(404).json({ error: 'Not Found' })
});

api.listen(3000, () => {
    console.log('Server started on port 3000')
});
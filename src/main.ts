import express from "express";
import cors from "cors";
import bodyParser from "body-parser";
import morgan from "morgan";
import helmet from "helmet"

import find from "./routes/find";
import cabra from "./routes/cabra";
import rateLimit from "./ratelimit/rllib";

const api = express();

api.use(cors())
api.use(bodyParser.urlencoded({ extended: true }))
api.use(bodyParser.json())
api.disable('x-powered-by')
api.use(morgan("dev"))

api.use(helmet({
    crossOriginEmbedderPolicy: false,
}));

api.use(
    rateLimit({
        windowMs: 30 * 1000,
        max: 10,
        legacyHeaders: true,
        standardHeaders: true,
        handler: (_, res) => res.status(429).send({ status: 429, message: "Too many requests" }),
        skipSuccessfulRequests: false,
        skipFailedRequests: true,
    }),
    rateLimit({
        windowMs: 15 * 60 * 1000,
        max: 500,
        legacyHeaders: true,
        standardHeaders: true,
        handler: (_, res) => res.status(429).send({ status: 429, message: "Too many requests" }),
        skipSuccessfulRequests: true,
        skipFailedRequests: false,
    }),
    rateLimit({
        windowMs: 5 * 60 * 1000,
        max: 5000,
        legacyHeaders: true,
        standardHeaders: true,
        handler: (_, res) => res.status(429).send({ status: 429, message: "Too many requests" }),
        skipSuccessfulRequests: false,
        skipFailedRequests: true,
    }))

api.use("/find", find);
api.use("/cabra", cabra)

api.use((req, res) => {
    res.status(404).json({ error: 'Not Found' })
});

api.listen(3000, () => {
    console.log('Server started on port 3000')
});
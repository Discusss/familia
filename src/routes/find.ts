import {Router} from "express";
import fs from "fs";

const router = Router();

// @ts-ignore
router.get('/', (req, res) => {
    try {
        const domain = req.get("host");
        const is_https = req.protocol === "https";
        const asset_size = fs.readdirSync('./assets/cabras').length;

        return res.json({
            message: "OK",
            url: `http${is_https ? "s" : ""}://${domain}/cabra/${Math.floor(Math.random() * asset_size)}`
        });
    } catch (e) {
        res.status(500).json({error: "Internal server error"});
    }
});

// @ts-ignore
router.get('/nutria', (req, res) => {

    const domain = req.get("host");
    const is_https = req.protocol === "https";
    const asset_size = fs.readdirSync('./assets/nutrias').length;

    return res.json({
        message: "OK",
        url: `http${is_https ? "s": ""}://${domain}/nutria/${Math.floor(Math.random() * asset_size)}`
    });

});

// @ts-ignore
router.get('/big-cat', (req, res) => {

    const domain = req.get("host");
    const is_https = req.protocol === "https";
    const asset_size = fs.readdirSync('./assets/big_cats').length;

    return res.json({
        message: "OK",
        url: `http${is_https ? "s": ""}://${domain}/big-cat/${Math.floor(Math.random() * asset_size)}`
    });

});

// @ts-ignore
router.get('/panda', (req, res) => {

    const domain = req.get("host");
    const is_https = req.protocol === "https";
    const asset_size = fs.readdirSync('./assets/pandas').length;

    return res.json({
        message: "OK",
        url: `http${is_https ? "s": ""}://${domain}/panda/${Math.floor(Math.random() * asset_size)}`
    });

});

export default router;
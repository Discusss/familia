import {Router} from "express";
import fs from "fs";

const router = Router();

router.get('/', (req, res) => {

    const domain = req.get("host");
    const is_https = req.protocol === "https";
    const asset_size = fs.readdirSync('./assets').length;

    return res.json({
        message: "OK",
        url: `http${is_https ? "s": ""}://${domain}/cabra/${Math.floor(Math.random() * asset_size)}`
    });

});

export default router;
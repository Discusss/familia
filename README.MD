# LA CABRA Image

This is a simple API to deliver random images various animals.

### Implemented animals
This API currently supports the following animals:
- Goats `goat`
- Otters `otter` (Thanks to users `@.griselda.` `@mhroom._` `@.kofu`)
- Pandas `panda` (Thanks to `@mhroom._`)
- Big Cats `big_cat` (Thanks to `@.griselda.`)

## Routes

### `/find/:animal`

- **Method**: `GET`
- **Description**: Returns a random image of the specified animal.

```json
{
  "message": "OK",
  "url": "<url to the image>"
}
```

### `/assets/:animal/:filename`
- **Method**: `GET`
- **Description**: Returns a static image of the specified animal.

## Self-hosting

### With Docker

You can download the image from the GitHub Container Registry and run it with Docker.

```bash
docker pull ghcr.io/discusss/lacabra-img:latest
```

You will need to set the following environment variables, either in a `.env` file or in the command line:
- `PORT`: The port on which the server will run. Default is `3000`.
- `HOST`: The host on which the server will run. Default is `0.0.0.0`
- `ENABLE_METRICS` (Optional): If set to `true`, the server will expose a `/metrics` endpoint with Prometheus metrics. Default is `false`.
- `METRICS_PASSWD` (Optional): The password to access the `/metrics` endpoint. Default is `lacabra`.
- `DOMAIN`: The domain of the server. Default is `localhost:3000`.
- `USE_HTTPS`: If set to `true`, the server will use HTTPS. Default is `false`.

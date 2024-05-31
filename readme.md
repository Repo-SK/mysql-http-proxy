## What is it
This is a simple web application to proxy HTTP traffic to your MySQL database.

I built this so I can use Cloudflare Workers to query certain databases.

It exposes a ``/query`` route
that you can ``POST`` an SQL query to.

The payload to ``POST`` is just a string:
```typescript
"SELECT * FROM users WHERE id = 1"
```

## Authentication
An ``Authentication`` header must be present otherwise all requests will be rejected.
The auth token can be set via the ``BEARER_TOKEN`` key in the env file.

## Compilation instructions:
* install cross
* run ``cross build --release --target x86_64-unknown-linux-gnu``

## Deployment instructions:
* Upload compiled binary to the internet
* ``cd /home``
* ``curl -o mysql-http-proxy <uploaded_file_url>``
* ``chmod +x mysql-http-proxy``
* make a systemd service and start it :)

## Example CURL request
```bash
curl -X POST http://<YOUR_SERVER_IP>:<YOUR_PORT>/query \
    -H "Authorization: Bearer <YOUR_BEARER_TOKEN>" \
    -H "Content-Type: application/json"  \
    -d '"SELECT * FROM users LIMIT 10"'
```
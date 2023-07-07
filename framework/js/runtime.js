const {
    WebSocketServer
} = require("ws");
const { app, ipcMain } = require("electron");

const app_id = process.env.electron_ws_port;
const pass = process.env.electron_ws_pass;
let connected = 0;
let last_disc = Date.now();

/**
 * @type {WebSocket}
 */
let ws = null;

ipcMain.on("data", (data) => {
    ws.send(JSON.stringify({
        ref: "ipc_listen",
        data
    }));
});

setInterval(() => {
    const now = Date.now();

    if (connected == 0 && now - last_disc >= 3 * 60 * 1000) {
        app.quit();
        process.exit(0);
    }
}, 3 * 60 * 1000);

app.on("ready", () => {
    try {
        const server = new WebSocketServer({
            port: app_id,
        });

        server.on("connection", (stream) => {
            connected++;

            let pwdclose = false;

            if (connected > 1) {
                app.quit();
            }

            ws = stream;
            stream.onclose = () => {
                connected--;
                last_disc = Date.now();
                if (!pwdclose) {
                    app.quit();
                }
            }
            stream.onerror = () => {
                connected--;
                app.quit();
            }
            stream.onmessage = (msg) => {
                try {
                    const data = JSON.parse(msg.data);

                    if (Number(data.pass) != pass) {
                        pwdclose = true;
                        stream.send(JSON.stringify({
                            ref: data.ref,
                            resp: "Invalid password"
                        }));
                        stream.close();
                        return;
                    }

                    try {
                        const resp = (require(`./${data.require}`))(data.data);

                        stream.send(JSON.stringify({
                           ref: data.ref,
                           resp, 
                        }));
                    } catch (e) {
                        stream.send(JSON.stringify({
                            ref: data.ref,
                            resp: e
                        }));
                    }
                } catch (_) {
                    app.quit();
                }
            }
        });
    } catch (_) {
        app.quit();
    }
});
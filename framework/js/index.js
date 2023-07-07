const {
    execSync
} = require("child_process");
const path = require("path");
const {
    WebSocket
} = require("ws");

let min = 49153;
let max = 65535;

let i = min;

function mkPassword() {
    let pwd = "";

    for (let i = 0; i < 150; i++) {
        pwd = String(Math.random()).replace(".", "0");
    }

    return pwd;
}

function runWs() {
    const ws = new WebSocket(`ws://localhost:${i}`, {
        timeout: 5
    });

    ws.onopen = () => {
        ws.close();
        i += 1;
        if ((i - 1) == max) {
            throw new Error("No open port found");
        }
        runWs();
    }
    ws.onerror = () => {
        const pass = mkPassword();
        const electron = path.join(__dirname, "node_modules", ".bin", "electron.cmd");

        execSync(`powershell $env:electron_ws_pass = ${pass};$env:electron_ws_port = ${i}; start-process -WindowStyle Hidden "${electron}" runtime.js`, console.log);
        console.log(i, pass);
    }
}

runWs();
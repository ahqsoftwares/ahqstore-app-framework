const { WebSocket } = require("ws");

const exec = require("child_process").execFileSync;

const currentDir = process.env.DBEUG
  ? "C:\\ProgramData\\AHQ Store Applications\\Programs\\unknown"
  : __dirname;

let dir = currentDir.split("\\");
dir.pop();
dir.pop();

const framework = dir.join("/") + "/Framework/framework.exe";

let pass = null;

let ws;

let listeners = 0;

const timer = setInterval(() => {
  if (ws) {
    ws.onclose = () => process.exit(0);
    ws.onerror = () => process.exit(0);

    clearInterval(timer);
  }
}, 5000);

/**
 * Tries to connect to AHQ Store Framework Agent
 * Resolves if it succeeds, rejects if it fails
 * @returns {Promise<void>}
 */
function start() {
  const [port, password] = String(exec(framework))
    .replace(/\n/g, "")
    .split(" ");

  pass = password;

  return new Promise((resolve, reject) => {
    ws = new WebSocket(`ws://localhost:${port}`);

    ws.onopen = () => resolve();
    ws.onerror = () => reject();
  });
}

/**
 * Listen to WebSocket Events
 * @param {Function} fn
 * @returns {number} total number of listeners
 */
function listenWs(fn) {
  if (!ws) throw new Error("Run start() first");

  ws.onmessage = fn;
  listeners++;
  return listeners;
}

/**
 * Sends a message to the electron app
 * @param {{
 *  require: string,
 *  ref: string,
 *  data: any
 * }} data 
 */
function sendWsMessage(data) {
  if (!pass) throw new Error("Run start() first");

  ws.send(JSON.stringify({
    pass,
    ...data
  }));
}

/**
 * Get the raw ws that is being wrapped
 * @returns {WebSocket}
 */
function getRawWs() {
  return ws;
}

module.exports = {
  start,
  listenWs,
  sendWsMessage,
  getRawWs
};

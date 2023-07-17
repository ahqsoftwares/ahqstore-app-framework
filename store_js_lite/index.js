/**
 * @type {string}
 */
const version = require("./package.json").version;

if (process.platform !== "win32") {
  throw new Error("Only Windows is supported");
}

/**
 * Use Debug Data
 * @returns {{
 *  start: Function,
 *  listenWs: Function,
 *  sendWsMessage: Function,
 *  getRawWs: Function
 * }}
 */
function debug() {
  return require("./src/debug");
}

module.exports = {
  version,
  ...require("./src/package"),
  debug
};

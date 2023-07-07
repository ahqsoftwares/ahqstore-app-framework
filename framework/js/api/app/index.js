const { app } = require("electron");

module.exports = (data) => {
    return eval(data.payload);
}
const { BrowserWindow } = require("electron");

module.exports = (data) => {
    const window = new BrowserWindow({
        ...data,
        webPreferences: {
            ...(data.webPreferences || {}),
            nodeIntegration: false,
            contextIsolation: false,
        }
    });

    return window.id;
}
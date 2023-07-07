const { BrowserWindow } = require("electron");

module.exports = (data) => {
    const win = BrowserWindow.getAllWindows().find(({ id }) => id == data.id);

    if (win != undefined) {
        win.loadURL(data.url);
        
        return data.url;
    } else {
        return {
            error: "Window not found"
        };
    }
}
const { BrowserWindow } = require("electron");

module.exports = (data) => {
    const win = BrowserWindow.getAllWindows().find(({ id }) => id == data.id);

    if (win != undefined) {
        win.loadFile(data.file);

        return data.file;
    } else {
        return {
            error: "Window not found"
        };
    }
}
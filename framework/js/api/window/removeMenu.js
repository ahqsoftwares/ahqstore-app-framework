const { BrowserWindow } = require("electron");

module.exports = (data) => {
    const win = BrowserWindow.getAllWindows().find(({ id }) => id == data.id);

    if (win != undefined) {
        win.removeMenu();

        return true;
    } else {
        return {
            error: "Window not found"
        };
    }
}
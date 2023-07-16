const { BrowserWindow } = require("electron");

module.exports = (data) => {
    const win = BrowserWindow.getAllWindows().find(({ id }) => id == data.id);

    if (win != undefined) {
        try {
            return win[data.property];
        } catch (_) {
            return {
                error: `Invalid / Unknown Property: ${data.property}`
            };
        }
    } else {
        return {
            error: "Window not found"
        };
    }
}
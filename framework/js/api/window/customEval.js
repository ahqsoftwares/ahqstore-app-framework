const { BrowserWindow } = require("electron");
const getMethods = require("../../getMethods");

module.exports = (data) => {
    const win = BrowserWindow.getAllWindows().find(({ id }) => id == data.id);

    if (win != undefined) {
        const methods = getMethods(win);

        if (methods.includes(data.method)) {
            return win[data.method](...data.args);
        } else {
            return {
                error: "Method not found"
            }; 
        }
    } else {
        return {
            error: "Window not found"
        };
    }
}
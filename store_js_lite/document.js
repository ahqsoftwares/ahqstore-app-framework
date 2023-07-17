const jsdoc2md = require("jsdoc-to-markdown");

const documentation = jsdoc2md.renderSync({
  files: ["index.js", "index.mjs", "src/*.js"],
});

const data = [
  "# AHQ Store JS.Lite",
  "A Core Package for interacting with the AHQ Store Framework (the framework uses electron under the hood)",
  "",
  "## Quick Example (Not Tested)",
  "```js",
  "const ahqstore = require(\"ahqstorejs.lite\");",
  "// If debug",
  "ahqstore.debug().start();",
  "",
  "// If Installed as an ahq store app",
  "ahqstore.start();",
  "```",
  documentation
].join("\n");

require("fs").writeFileSync("./README.md", data);

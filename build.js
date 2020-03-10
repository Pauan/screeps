const $fs = require("fs");
const $path = require("path");

//const dir = "C:\\Users\\Pauan\\AppData\\Local\\Screeps\\scripts\\screeps.com\\default";

let js = $fs.readFileSync("pkg/screeps.js", { encoding: "utf8" });

js = js.replace(/const path = require\('path'\)\.join\(__dirname, 'screeps_bg\.wasm'\);/, "");
js = js.replace(/const bytes = require\('fs'\)\.readFileSync\(path\);/, "const bytes = require('screeps_bg');");

$fs.writeFileSync("pkg/screeps.js", js);

// .replace(/const \{ TextDecoder \} = require\(String\.raw`util`\);\n/, "").replace(/new TextDecoder/, "new window.TextDecoder")
//$fs.writeFileSync($path.join(dir, "main.js"), js);
//$fs.writeFileSync($path.join(dir, "screeps_bg.wasm"), $fs.readFileSync("pkg/screeps_bg.wasm"));

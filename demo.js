const { generate } = require("./index");
const fs = require("fs");
const path = require("path");

// read ./fixtures/Button.swift
const content = fs.readFileSync("./fixtures/controls/DatePicker.swift", "utf8");
const outdir = path.resolve(__dirname, "output");

// rm output directory
if (fs.existsSync(outdir)) {
  fs.rmSync(outdir, { recursive: true });
}

generate(content, outdir, false);
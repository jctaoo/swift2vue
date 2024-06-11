const { generate } = require("./index");
const fs = require("fs");

// read ./fixtures/Button.swift
const content = fs.readFileSync("./fixtures/Button.swift", "utf8");

generate(content);
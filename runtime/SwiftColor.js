// SwiftColor(SwiftColor.sRGB, {red: 0.98, green: 0.9, blue: 0.2})

const RGBColorSpace = {
  sRGB: 0,
  sRGBLinear: 1,
  displayP3: 2
};

class _SwiftColor {
  static fromHex(hex) {
    // rgb value from 0 to 1
    const hexValue = parseInt(hex.replace(/^#/, ''), 16);
    const red = ((hexValue >> 16) & 0xff) / 255;
    const green = ((hexValue >> 8) & 0xff) / 255;
    const blue = (hexValue & 0xff) / 255;
    return new _SwiftColor(RGBColorSpace.sRGB, { red, green, blue });
  }

  toHex() {
    const red = Math.round(this.red * 255);
    const green = Math.round(this.green * 255);
    const blue = Math.round(this.blue * 255);
    return `#${((1 << 24) + (red << 16) + (green << 8) + blue).toString(16).slice(1)}`;
  }

  constructor(colorSpace, color) {
    this.colorSpace = colorSpace;
    this.color = color;
  }

  get red() {
    return this.color.red;
  }
  set red(value) {
    this.color.red = value;
  }

  get green() {
    return this.color.green;
  }
  set green(value) {
    this.color.green = value;
  }

  get blue() {
    return this.color.blue;
  }
  set blue(value) {
    this.color.blue = value;
  }
};

function SwiftColor(colorSpace, color) {
  return new _SwiftColor(colorSpace, color);
}

SwiftColor.sRGB = RGBColorSpace.sRGB;
SwiftColor.sRGBLinear = RGBColorSpace.sRGBLinear;
SwiftColor.displayP3 = RGBColorSpace.displayP3;
SwiftColor.fromHex = _SwiftColor.fromHex;

export default SwiftColor;
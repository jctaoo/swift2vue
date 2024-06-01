let ENABLE_COLLECT = false;

function uniqueId() {
  // more complex
  return Math.random().toString(36).substr(2, 9);
}

function makeCollector() {
  let collect = [];
  return {
    push: (value) => {
      collect.push(value);
    },
    consume: () => {
      const result = [...collect];
      collect = [];
      return result;
    },
  }
}
const collector = makeCollector();

class View {
  static TAG = "view";
  _padding = null;
  _background = null;
  _foregroundStyle = null;
  children = [];

  constructor() {
    if (ENABLE_COLLECT) {
      collector.push(this);
    }
  }

  _constructBuilder(builder) {
    ENABLE_COLLECT = true;
    builder();
    ENABLE_COLLECT = false;

    this.children = collector.consume();
    // setTimeout(() => {
    //   this.children = collector.consume();
    // }, 0);
  }

  padding(value) {
    this._padding = value;
    return this;
  }

  background(value) {
    this._background = value;
    return this;
  }

  foregroundStyle(value) {
    this._foregroundStyle = value;
    return this;
  }

  rendererScript() {
    return this.children.map((child) => child.rendererScript()).join("\n");
  }

  renderParam() {
    return [
      this._padding ? `padding="${this._padding}"` : "",
      this._background ? `background="${this._background}"` : "",
      this._foregroundStyle ? `foregroundStyle="${this._foregroundStyle}"` : "",
    ].filter(Boolean).join(" ");
  }

  renderStartTag() {
    const param = this.renderParam();
    const paramString = param ? ` ${param}` : "";
    return `<${this.constructor.TAG}${paramString}>`;
  }

  renderContent() {
    return "\n" + this.children.map((child) => child.render()).map(i => `  ${i}`).join("\n") + "\n";
  }

  render() {
    const children = this.renderContent();
    return `
${this.renderStartTag()}${children}</${this.constructor.TAG}>
    `.trim();
  }
}

class VStack extends View {
  static TAG = "v-stack";

  constructor(builder) {
    super();
    this._constructBuilder(builder);
  }
}

class Text extends View {
  static TAG = "swift-text";
  _font = null;

  constructor(value) {
    super();
    this.value = value;
  }

  font(value) {
    this._font = value;
    return this;
  }

  renderContent() {
    return this.value;
  }
}

class Button extends View {
  static TAG = "swift-button";

  constructor(value, onClick) {
    super();
    this.value = value;
    this.onClick = onClick;
  }

  renderContent() {
    return this.value;
  }

  rendererScript() {
    return `(${this.onClick})()`;
  }
}

const fontPreExp = {
  largeTitle: "largeTitle",
  title: "title",
  title2: "title2",
  title3: "title3",
  headline: "headline",
  subheadline: "subheadline",
  body: "body",
};

const Color = {
  red: "red",
  blue: "blue",
  green: "green",
}

const foregroundStylePreExp = Color;

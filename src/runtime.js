let ENABLE_COLLECT = 0;
let GENETAED_IDS = new Set();

function uniqueId() {
    const code = 'a' + Math.floor(Math.random() * 1000);
    if (GENETAED_IDS.has(code)) {
        return uniqueId();
    }
    GENETAED_IDS.add(code);
    return code;
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
    };
}

let currentCollector = null;

class $View {
    static TAG = "div";
    _padding = null;
    _background = null;
    _foregroundStyle = null;
    children = [];

    constructor() {
        this.id = uniqueId();
        if (ENABLE_COLLECT > 0) {
            currentCollector?.push(this);
            if (!!currentCollector) {
                // console.log("inserted: ", this.constructor.name)
            } else {
                // console.log("no collector", this.constructor.name)
            }
        } else {
            // console.log("no collector", this.constructor.name)
        }
    }

    _constructBuilder(builder) {
        let oldCollector = currentCollector;
        ENABLE_COLLECT ++;
        currentCollector = makeCollector();
        builder();
        ENABLE_COLLECT --;

        this.children = currentCollector.consume();
        currentCollector = oldCollector;

        // console.log("collected children: ", this.children, this.constructor.name)

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
            this._padding ? `data-padding="${this._padding}"` : "",
            this._background ? `data-background="${this._background}"` : "",
            this._foregroundStyle
                ? `data-foregroundStyle="${this._foregroundStyle}"`
                : "",
        ]
            .filter(Boolean)
            .join(" ");
    }

    renderStartTag() {
        const param = this.renderParam();
        const paramString = param ? ` ${param}` : "";
        return `<${this.constructor.TAG}${paramString}>`;
    }

    renderContent() {
        // console.log(this.children)
        return (
            "\n" +
            this.children
                .map((child) => child.render())
                .map((i) => `  ${i}`)
                .join("\n") +
            "\n"
        );
    }

    render() {
        const children = this.renderContent();
        return `
${this.renderStartTag()}${children}</${this.constructor.TAG}>
    `.trim();
    }
}

class $VStack extends $View {
    static TAG = "div";

    renderParam() {
        return super.renderParam() + ' class="vstack"';
    }

    constructor(builder) {
        super();
        this._constructBuilder(builder);
    }
}

class $HStack extends $View {
    static TAG = "div";

    renderParam() {
        return super.renderParam() + ' class="hstack"';
    }

    constructor(builder) {
        super();
        this._constructBuilder(builder);
    }
}

class $Text extends $View {
    static TAG = "span";
    _font = null;

    constructor(value) {
        super();
        this.value = value;
    }

    font(value) {
        this._font = value;
        return this;
    }

    renderParam() {
        return [
            super.renderParam(),
            this._font ? `data-font="${this._font}"` : "",
            `class="text"`,
        ]
            .filter(Boolean)
            .join(" ");
    }

    renderContent() {
        return this.value;
    }
}

class $Button extends $View {
    static TAG = "button";

    constructor(value, onClick) {
        super();
        this.value = value;
        this.onClick = onClick;
    }

    renderContent() {
        return this.value;
    }

    renderParam() {
        return [super.renderParam(), `class="button"`, `@click="${this.id}OnClick"`].filter(Boolean).join(" ");
    }

    rendererScript() {
        const fn = `
            ${this.id}OnClick: () => {
                (${this.onClick})();
            },
        `
        return fn;
    }
}

function VStack(builder) {
    return new $VStack(builder);
}

function HStack(builder) {
    return new $HStack(builder);
}

function Text(value) {
    return new $Text(value);
}

function Button(value, onClick) {
    return new $Button(value, onClick);
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
    yellow: "yellow",
};

const foregroundStylePreExp = Color;
const foregroundColorPreExp = Color;
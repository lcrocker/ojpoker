#!/usr/bin/env -S deno --allow-write=. build_cards.ts

import { SUITS } from "./suits.ts";
import { RANKS } from "./ranks.ts";
import { FACE_JS } from "./face_js.ts";

/*
 * Build set of Crocker/Dovgan cards with given options.
 */

type GlobalOptions = {
    outerBorder: boolean;
    outerBorderWidth: number;
    outerBorderRadius: number;
    outerBorderStrokeColor: string;
    outerBorderFillColor: string;
    innerBorder: boolean;
    innerBorderWidth: number;
    innerBorderRadius: number;
    innerBorderStrokeColor: string;
    innerBorderFillColor: string;
    clubColor: string;
    diamondColor: string;
    heartColor: string;
    spadeColor: string;
}

const globalOptions: GlobalOptions = {
    outerBorder: true,
    outerBorderWidth: 6,
    outerBorderRadius: 25,
    outerBorderStrokeColor: "#000",
    outerBorderFillColor: "#fff",
    innerBorder: true,
    innerBorderWidth: 6,
    innerBorderRadius: 20,
    innerBorderStrokeColor: "#000",
    innerBorderFillColor: "#ffc",
    clubColor: "#0c0",
    diamondColor: "#00c",
    heartColor: "#c00",
    spadeColor: "#000",
};

type LayerOptions = {
    x: number;
    y: number;
    scale: number;
    stroke: string;
    fill: string;
}

class Layer {
    template: string = "";
    options: LayerOptions = {
        x: 0,
        y: 0,
        scale: 1.0,
        stroke: "#000",
        fill: "#fff",
    };

    constructor(template: string, opts: Partial<LayerOptions>) {
        this.template = template;
        this.options = { ...this.options, ...opts };
    }

    output(): string {
        let out: string = this.template.replaceAll("{{x}}", this.options.x.toString());
        out = out.replaceAll("{{y}}", this.options.y.toString());
        out = out.replaceAll("{{scale}}", this.options.scale.toString());
        out = out.replaceAll("{{stroke}}", this.options.stroke);
        out = out.replaceAll("{{fill}}", this.options.fill);
        return out;
    }
}

class OuterBorder extends Layer {
    constructor() {
        super(
`  <rect x="${globalOptions.outerBorderWidth/2}" y="${globalOptions.outerBorderWidth/2}"
  width="${1000-globalOptions.outerBorderWidth}" height="${1400-globalOptions.outerBorderWidth}"
  rx="${globalOptions.outerBorderRadius}" ry="${globalOptions.outerBorderRadius}"
  fill="${globalOptions.outerBorderFillColor}" stroke="${globalOptions.outerBorderStrokeColor}"
  stroke-width="${globalOptions.outerBorderWidth}px" />
`, {});
    }
}

class InnerBorder extends Layer {
    constructor() {
        super(
`  <rect x="260" y="228" width="720" height="1152"
  rx="${globalOptions.innerBorderRadius}" ry="${globalOptions.innerBorderRadius}"
  fill="${globalOptions.innerBorderFillColor}" stroke="${globalOptions.innerBorderStrokeColor}"
  stroke-width="${globalOptions.innerBorderWidth}px" />
`, {});
    }
}

class Pip extends Layer {
    constructor(suit: string, x: number, y: number, opts: Partial<LayerOptions>) {
        let path: string = "";
        let color: string = "";

        switch (suit) {
            case "c":
                path = SUITS.club;
                if (opts.fill) { globalOptions.clubColor = opts.fill; }
                color = globalOptions.clubColor;
                break;
            case "d":
                path = SUITS.diamond;
                if (opts.fill) { globalOptions.diamondColor = opts.fill; }
                color = globalOptions.diamondColor;
                break;
            case "h":
                path = SUITS.heart;
                if (opts.fill) { globalOptions.heartColor = opts.fill; }
                color = globalOptions.heartColor;
                break;
            case "s":
                path = SUITS.spade;
                if (opts.fill) { globalOptions.spadeColor = opts.fill; }
                color = globalOptions.spadeColor;
                break;
            default:
                console.assert(false);
        }
        // The suits are all in a 50x50 box, so the inner translation here
        // moves the origin to the center of the pip, because they are
        // most usefully aligned by center.
        super(
`  <g transform="translate({{x}},{{y}}) scale({{scale}},{{scale}})">
    <path transform="translate(-25,-25)"
    d="${path}" fill="{{fill}}" stroke="none" />
  </g>
`,      {
            x: x,
            y: y,
            scale: opts.scale ? opts.scale : 1.0,
            fill: color,
        });
    }
}

class IndexPipVertical extends Pip {
    constructor(suit: string, color?: string) {
        super(suit, 131, 410, { scale: 5.0, fill: color });
    }
}

class IndexPipHorizontal extends Pip {
    constructor(suit: string, color?: string) {
        super(suit, 384, 118, { scale: 5.0, fill: color } );
    }
}

class IndexRank extends Layer {
    constructor(rank: string, suit: string) {
        let path: string = "";
        let color: string = "#888";

        switch (rank) {
            case "A":
                path = RANKS.ace;
                break;
            case "2":
                path = RANKS.deuce;
                break;
            case "3":
                path = RANKS.trey;
                break;
            case "4":
                path = RANKS.four;
                break;
            case "5":
                path = RANKS.five;
                break;
            case "6":
                path = RANKS.six;
                break;
            case "7":
                path = RANKS.seven;
                break;
            case "8":
                path = RANKS.eight;
                break;
            case "9":
                path = RANKS.nine;
                break;
            case "T":
                path = RANKS.ten;
                break;
            case "J":
                path = RANKS.jack;
                break;
            case "C":
                path = RANKS.knight;
                break;
            case "Q":
                path = RANKS.queen;
                break;
            case "K":
                path = RANKS.king;
                break;
            case "Jk":
                path = RANKS.joker;
                break;
            default:
                console.assert(false);
        }
        switch (suit) {
            case "c":
                color = globalOptions.clubColor;
                break;
            case "d":
                color = globalOptions.diamondColor;
                break;
            case "h":
                color = globalOptions.heartColor;
                break;
            case "s":
                color = globalOptions.spadeColor;
                break;
            default:
                console.assert(false);
        }
        super(
`  <path transform="translate({{x}},{{y}}) scale({{scale}},{{scale}})"
    d="${path}" fill="{{fill}}" stroke="none" />
`, { x: 12, y: 12, scale: 1.0, fill: color } );
    }
}

class Face extends Layer {
    constructor(card: string) {
        let path: string = "";

        switch (card) {
            case "Js":
                path = FACE_JS;
                break;
            default:
                console.assert(false);
        }
        super(
`    <g transform="translate({{x}},{{y}}) scale({{scale}},{{scale}})">
      ${path}
    </g>
`, { x: 16, y: 12, scale: 0.625 } );
    }
}

function buildCard(): string {
    const outerLayers: Layer[] = [];
    const innerLayers: Layer[] = [];

    outerLayers.push(new OuterBorder());
    outerLayers.push(new IndexRank("J", "s"));
    outerLayers.push(new IndexPipVertical("s"));
    outerLayers.push(new IndexPipHorizontal("s"));
    outerLayers.push(new InnerBorder());

    // innerLayers.push(new Pip("club", 500, 800, 12));
    innerLayers.push(new Face("Js"));

    let out: string =
`<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg
  width="1000px"
  height="1400px"
  viewBox="0 0 1000 1400"
  version="1.1"
  id="svg1"
  xmlns="http://www.w3.org/2000/svg"
  xmlns:svg="http://www.w3.org/2000/svg">
`;
    for (const layer of outerLayers) {
        out += layer.output();
    }
    // Inner box will have 1000x1600 virtual coordinates
    out += `
  <g transform="translate(260,228) scale(0.72,0.72)">
`;
    for (const layer of innerLayers) {
        out += layer.output();
    }
    out +=
`  </g>
</svg>
`;
    return out;
}

if (import.meta.main) {
    const svg = buildCard();
    console.log(svg);
}

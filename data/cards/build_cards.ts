#!/usr/bin/env -S deno --allow-write=. build_cards.ts

import { SUITS } from "./suits.ts";
import { RANKS } from "./ranks.ts";
import { FACE_JC } from "./face_jc.ts";
import { FACE_JD } from "./face_jd.ts";
import { FACE_JH } from "./face_jh.ts";
import { FACE_JS } from "./face_js.ts";
import { DECORATION_AS } from "./decoration_as.ts";

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
    innerPipScale1: number;
    innerPipScale2: number;
    acePipScale: number;
    indexPipScale: number;
    clubColor: string;
    diamondColor: string;
    heartColor: string;
    spadeColor: string;
    clubAdjustment: number;
    diamondAdjustment: number;
    heartAdjustment: number;
    spadeAdjustment: number;
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
    innerPipScale1: 8,
    innerPipScale2: 7,
    indexPipScale: 4,
    acePipScale: 12,
    clubColor: "#0b0",
    diamondColor: "#00b",
    heartColor: "#b00",
    spadeColor: "#000",
    clubAdjustment: 0.95,
    diamondAdjustment: 1.05,
    heartAdjustment: 0.88,
    spadeAdjustment: 1.0,
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
                opts.scale = opts.scale ? opts.scale * globalOptions.clubAdjustment : 1.0;
                break;
            case "d":
                path = SUITS.diamond;
                if (opts.fill) { globalOptions.diamondColor = opts.fill; }
                color = globalOptions.diamondColor;
                opts.scale = opts.scale ? opts.scale * globalOptions.diamondAdjustment : 1.0;
                break;
            case "h":
                path = SUITS.heart;
                if (opts.fill) { globalOptions.heartColor = opts.fill; }
                color = globalOptions.heartColor;
                opts.scale = opts.scale ? opts.scale * globalOptions.heartAdjustment : 1.0;
                break;
            case "s":
                path = SUITS.spade;
                if (opts.fill) { globalOptions.spadeColor = opts.fill; }
                color = globalOptions.spadeColor;
                opts.scale = opts.scale ? opts.scale * globalOptions.spadeAdjustment : 1.0;
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

class InnerPips {
    rank: string;
    suit: string;

    constructor(rank: string, suit: string) {
        this.rank = rank;
        this.suit = suit;
    }

    *ace_center(height: number): Generator<Layer> {
        yield new Pip(this.suit, 500, height, { scale: globalOptions.acePipScale });
    }

    *deuce_outer(): Generator<Layer> {
        yield new Pip(this.suit, 500, 320, { scale: globalOptions.innerPipScale1 });
        yield new Pip(this.suit, 500, 1280, { scale: globalOptions.innerPipScale1 });
    }

    *trey_center(): Generator<Layer> {
        yield new Pip(this.suit, 500, 800, { scale: globalOptions.innerPipScale1 });
    }

    *four_outer(): Generator<Layer> {
        yield new Pip(this.suit, 250, 320, { scale: globalOptions.innerPipScale1 });
        yield new Pip(this.suit, 250, 1280, { scale: globalOptions.innerPipScale1 });
        yield new Pip(this.suit, 750, 320, { scale: globalOptions.innerPipScale1 });
        yield new Pip(this.suit, 750, 1280, { scale: globalOptions.innerPipScale1 });
    }

    *six_center(): Generator<Layer> {
        yield new Pip(this.suit, 250, 800, { scale: globalOptions.innerPipScale1 });
        yield new Pip(this.suit, 750, 800, { scale: globalOptions.innerPipScale1 });
    }

    *seven_center(): Generator<Layer> {
        yield new Pip(this.suit, 500, 560, { scale: globalOptions.innerPipScale1 });
    }

    *eight_center(): Generator<Layer> {
        yield new Pip(this.suit, 500, 560, { scale: globalOptions.innerPipScale1 });
        yield new Pip(this.suit, 500, 1040, { scale: globalOptions.innerPipScale1 });
    }

    *eight_outer(): Generator<Layer> {
        yield new Pip(this.suit, 250, 200, { scale: globalOptions.innerPipScale2 });
        yield new Pip(this.suit, 250, 600, { scale: globalOptions.innerPipScale2 });
        yield new Pip(this.suit, 250, 1000, { scale: globalOptions.innerPipScale2 });
        yield new Pip(this.suit, 250, 1400, { scale: globalOptions.innerPipScale2 });
        yield new Pip(this.suit, 750, 200, { scale: globalOptions.innerPipScale2 });
        yield new Pip(this.suit, 750, 600, { scale: globalOptions.innerPipScale2 });
        yield new Pip(this.suit, 750, 1000, { scale: globalOptions.innerPipScale2 });
        yield new Pip(this.suit, 750, 1400, { scale: globalOptions.innerPipScale2 });
    }

    *nine_center(): Generator<Layer> {
        yield new Pip(this.suit, 500, 800, { scale: globalOptions.innerPipScale2 });
    }

    *ten_center(): Generator<Layer> {
        yield new Pip(this.suit, 500, 400, { scale: globalOptions.innerPipScale2 });
        yield new Pip(this.suit, 500, 1200, { scale: globalOptions.innerPipScale2 });
    }

    *iter(): Generator<Layer> {
        switch (this.rank) {
            case "A":
                if ("s" == this.suit) {
                    yield* this.ace_center(775);
                    yield new AceDecoration(825);
                } else {
                    yield* this.ace_center(800);
                }
                break;
            case "2":
                yield* this.deuce_outer();
                break;
            case "3":
                yield* this.deuce_outer();
                yield* this.trey_center();
                break;
            case "4":
                yield* this.four_outer();
                break;
            case "5":
                yield* this.four_outer();
                yield* this.trey_center();
                break;
            case "6":
                yield* this.four_outer();
                yield* this.six_center();
                break;
            case "7":
                yield* this.four_outer();
                yield* this.six_center();
                yield* this.seven_center();
                break;
            case "8":
                yield* this.four_outer();
                yield* this.six_center();
                yield* this.eight_center();
                break;
            case "9":
                yield* this.eight_outer();
                yield* this.nine_center();
                break;
            default:
                console.assert(this.rank == "T");
                yield* this.eight_outer();
                yield* this.ten_center();
                break;
        }
    }
}

class IndexPipVertical extends Pip {
    constructor(suit: string, color?: string) {
        super(suit, 131, 438, { scale: globalOptions.indexPipScale, fill: color });
    }
}

class IndexPipHorizontal extends Pip {
    constructor(suit: string, color?: string) {
        super(suit, 382, 118, { scale: globalOptions.indexPipScale, fill: color } );
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
`, { x: 12, y: 40, scale: 1.0, fill: color } );
    }
}

class AceDecoration extends Layer {
    constructor(height: number) {
        super(
`  <g transform="translate({{x}},{{y}}) scale({{scale}},{{scale}})">
    ${DECORATION_AS}
  </g>
`, { x: 500, y: height, scale: 0.9 } );
    }
}

class Face extends Layer {
    constructor(card: string) {
        let path: string = "";

        switch (card) {
            case "Jc":
                path = FACE_JC;
                break;
            case "Jd":
                path = FACE_JD;
                break;
            case "Jh":
                path = FACE_JH;
                break;
            case "Js":
                path = FACE_JS;
                break;
            default:
                path = "";
        }
        super(
`    <g transform="translate({{x}},{{y}}) scale({{scale}},{{scale}})">
      ${path}
    </g>
`, { x: 500, y: 800, scale: 1 } );
    }
}

function buildCard(card: string, x: number, y: number): string {
    const rank = card[0];
    const suit = card[1];
    const outerLayers: Layer[] = [];
    const innerLayers: Layer[] = [];

    outerLayers.push(new OuterBorder());
    outerLayers.push(new IndexRank(rank, suit));
    outerLayers.push(new IndexPipVertical(suit));
    outerLayers.push(new IndexPipHorizontal(suit));
    outerLayers.push(new InnerBorder());

    if (["J", "C", "Q", "K"].includes(rank)) {
        innerLayers.push(new Face(card));
    } else {
        const fp = new InnerPips(rank, suit);
        for (const layer of fp.iter()) {
            innerLayers.push(layer);
        }
    }

    let out: string =
`<svg transform="translate(${x*1100},${y*1500}) scale(1.0,1.0)"
  width="1000px"
  height="1400px">
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

function buildCardDeck(): string {
    let out: string =
`<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg
  width="15300px"
  height="6000px"
  viewBox="0 0 15300 6000"
  version="1.1"
  id="svg1"
  xmlns="http://www.w3.org/2000/svg"
  xmlns:svg="http://www.w3.org/2000/svg">
`;
out += buildCard("2c", 0, 0);
out += buildCard("2d", 0, 1);
out += buildCard("2h", 0, 2);
out += buildCard("2s", 0, 3);

out += buildCard("3c", 1, 0);
out += buildCard("3d", 1, 1);
out += buildCard("3h", 1, 2);
out += buildCard("3s", 1, 3);

out += buildCard("4c", 2, 0);
out += buildCard("4d", 2, 1);
out += buildCard("4h", 2, 2);
out += buildCard("4s", 2, 3);

out += buildCard("5c", 3, 0);
out += buildCard("5d", 3, 1);
out += buildCard("5h", 3, 2);
out += buildCard("5s", 3, 3);

out += buildCard("6c", 4, 0);
out += buildCard("6d", 4, 1);
out += buildCard("6h", 4, 2);
out += buildCard("6s", 4, 3);

out += buildCard("7c", 5, 0);
out += buildCard("7d", 5, 1);
out += buildCard("7h", 5, 2);
out += buildCard("7s", 5, 3);

out += buildCard("8c", 6, 0);
out += buildCard("8d", 6, 1);
out += buildCard("8h", 6, 2);
out += buildCard("8s", 6, 3);

out += buildCard("9c", 7, 0);
out += buildCard("9d", 7, 1);
out += buildCard("9h", 7, 2);
out += buildCard("9s", 7, 3);

out += buildCard("Tc", 8, 0);
out += buildCard("Td", 8, 1);
out += buildCard("Th", 8, 2);
out += buildCard("Ts", 8, 3);

out += buildCard("Jc", 9, 0);
out += buildCard("Jd", 9, 1);
out += buildCard("Jh", 9, 2);
out += buildCard("Js", 9, 3);

out += buildCard("Cc", 10, 0);
out += buildCard("Cd", 10, 1);
out += buildCard("Ch", 10, 2);
out += buildCard("Cs", 10, 3);

out += buildCard("Qc", 11, 0);
out += buildCard("Qd", 11, 1);
out += buildCard("Qh", 11, 2);
out += buildCard("Qs", 11, 3);

out += buildCard("Kc", 12, 0);
out += buildCard("Kd", 12, 1);
out += buildCard("Kh", 12, 2);
out += buildCard("Ks", 12, 3);

out += buildCard("Ac", 13, 0);
out += buildCard("Ad", 13, 1);
out += buildCard("Ah", 13, 2);
out += buildCard("As", 13, 3);

out +=
`</svg>
`;
    return out;
}

if (import.meta.main) {
    const svg = buildCardDeck();
    console.log(svg);
}

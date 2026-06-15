// Generates a 1024x1024 brand PNG for RoPresence with zero dependencies
// (pure Node: manual PNG encoding via the built-in zlib).
// Run: node scripts/gen-icon.mjs  ->  writes scripts/app-icon.png
// Then: pnpm tauri icon scripts/app-icon.png  (expands to all platform icons)

import { deflateSync } from "node:zlib";
import { writeFileSync, mkdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const N = 1024;

// --- helpers ---------------------------------------------------------------
const clamp = (v, a, b) => Math.max(a, Math.min(b, v));
const lerp = (a, b, t) => a + (b - a) * t;
const smoothstep = (e0, e1, x) => {
  const t = clamp((x - e0) / (e1 - e0), 0, 1);
  return t * t * (3 - 2 * t);
};
const mix = (c1, c2, t) => [
  lerp(c1[0], c2[0], t),
  lerp(c1[1], c2[1], t),
  lerp(c1[2], c2[2], t),
];

// Brand colors
const TOP = [70, 165, 255]; // bright accent blue
const BOTTOM = [20, 70, 175]; // deep blue
const RING = [242, 245, 249]; // near-white
const ONLINE = [59, 165, 93]; // status green

const buf = Buffer.alloc(N * N * 4);

const c = N / 2;
const cornerR = N * 0.22;

// Rounded-rect signed coverage (1 inside, 0 outside, smooth edge)
function roundedRectCoverage(x, y) {
  const margin = N * 0.04;
  const half = N / 2 - margin;
  const dx = Math.abs(x - c) - (half - cornerR);
  const dy = Math.abs(y - c) - (half - cornerR);
  const outside =
    Math.hypot(Math.max(dx, 0), Math.max(dy, 0)) +
    Math.min(Math.max(dx, dy), 0) -
    cornerR;
  return 1 - smoothstep(-1.5, 1.5, outside);
}

// Avatar ring (donut) centered, slightly up-left to leave room for status dot
const ringCx = c - N * 0.015;
const ringCy = c - N * 0.015;
const ringR = N * 0.255;
const ringW = N * 0.072;

// Online status dot bottom-right
const dotCx = c + N * 0.205;
const dotCy = c + N * 0.205;
const dotR = N * 0.105;
const dotGap = N * 0.03; // dark separation ring around dot

for (let y = 0; y < N; y++) {
  for (let x = 0; x < N; x++) {
    const i = (y * N + x) * 4;

    // Background gradient (top -> bottom) with a soft radial glow near top.
    const tg = y / N;
    let col = mix(TOP, BOTTOM, tg);
    const glow = smoothstep(N * 0.7, 0, Math.hypot(x - c, y - N * 0.18));
    col = mix(col, [120, 200, 255], glow * 0.25);

    // Ring (white avatar outline)
    const dRing = Math.abs(Math.hypot(x - ringCx, y - ringCy) - ringR);
    const ringCov = 1 - smoothstep(ringW / 2 - 1.5, ringW / 2 + 1.5, dRing);
    col = mix(col, RING, ringCov);

    // Dark gap then green online dot (drawn on top of everything)
    const dDot = Math.hypot(x - dotCx, y - dotCy);
    const gapCov = 1 - smoothstep(dotR + dotGap - 1.5, dotR + dotGap + 1.5, dDot);
    col = mix(col, mix(TOP, BOTTOM, tg), gapCov); // carve background-colored gap
    const dotCov = 1 - smoothstep(dotR - 1.5, dotR + 1.5, dDot);
    col = mix(col, ONLINE, dotCov);

    const cov = roundedRectCoverage(x, y);
    buf[i] = Math.round(clamp(col[0], 0, 255));
    buf[i + 1] = Math.round(clamp(col[1], 0, 255));
    buf[i + 2] = Math.round(clamp(col[2], 0, 255));
    buf[i + 3] = Math.round(clamp(cov, 0, 1) * 255);
  }
}

// --- PNG encoding ----------------------------------------------------------
function crc32(buf) {
  let crc = 0xffffffff;
  for (let i = 0; i < buf.length; i++) {
    crc ^= buf[i];
    for (let k = 0; k < 8; k++) {
      crc = crc & 1 ? (crc >>> 1) ^ 0xedb88320 : crc >>> 1;
    }
  }
  return (crc ^ 0xffffffff) >>> 0;
}

function chunk(type, data) {
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length, 0);
  const typeBuf = Buffer.from(type, "ascii");
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(Buffer.concat([typeBuf, data])), 0);
  return Buffer.concat([len, typeBuf, data, crc]);
}

// Raw image: each scanline prefixed with filter byte 0
const raw = Buffer.alloc(N * (N * 4 + 1));
for (let y = 0; y < N; y++) {
  raw[y * (N * 4 + 1)] = 0;
  buf.copy(raw, y * (N * 4 + 1) + 1, y * N * 4, (y + 1) * N * 4);
}

const ihdr = Buffer.alloc(13);
ihdr.writeUInt32BE(N, 0);
ihdr.writeUInt32BE(N, 4);
ihdr[8] = 8; // bit depth
ihdr[9] = 6; // color type RGBA
ihdr[10] = 0;
ihdr[11] = 0;
ihdr[12] = 0;

const png = Buffer.concat([
  Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
  chunk("IHDR", ihdr),
  chunk("IDAT", deflateSync(raw, { level: 9 })),
  chunk("IEND", Buffer.alloc(0)),
]);

const here = dirname(fileURLToPath(import.meta.url));
mkdirSync(here, { recursive: true });
const out = join(here, "app-icon.png");
writeFileSync(out, png);
console.log(`Wrote ${out} (${png.length} bytes, ${N}x${N})`);

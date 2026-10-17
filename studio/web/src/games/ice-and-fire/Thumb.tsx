import type { ReactElement } from 'react';

/* A fixed picture of a game in progress for the home page's card, in the style of the other
   cards: no game logic, only a drawing. Fire (red) spreads from the top-left HQ, ice (blue) from
   the bottom-right one, and they meet in the middle of a map with holes. */

/* The top half of the map, rows y = 0 to 5: `#` void, `.` neutral, `R` red, `B` blue, `F` red's
   HQ, `1`-`3` a red unit of that level, `a`-`c` a blue unit of level 1 to 3. The bottom half is
   the top half turned by half a turn with the colours swapped, as the game's maps are
   symmetric. */
const TOP = [
  'FRRRRR..#.##',
  'RRR2RRR.#..#',
  'R1RRRRRR.#..',
  'RRRRRRR3...#',
  '#RRRRR1BB..#',
  '##RRRRbBB.##',
];

const SIZE = 12;
const GAP = 1;
const CELL = (90 - GAP * (SIZE + 1)) / SIZE;

/** The map, row by row: the top half and its half-turn with red and blue swapped. */
function rows(): string[] {
  const swap: Record<string, string> = {
    R: 'B',
    B: 'R',
    F: 'I',
    '1': 'a',
    '2': 'b',
    '3': 'c',
    a: '1',
    b: '2',
    c: '3',
  };
  const bottom = [...TOP].reverse().map((row) =>
    [...row]
      .reverse()
      .map((ch) => swap[ch] ?? ch)
      .join(''),
  );
  return [...TOP, ...bottom];
}

const RED = '#c62f26';
const BLUE = '#1d5fc4';
const RED_TILE = '#e9a39b';
const BLUE_TILE = '#9ec3ef';

/** A flame, centred on (cx, cy), about `s` tall. */
function flame(cx: number, cy: number, s: number) {
  const h = s / 2;
  return `M${cx} ${cy - h}C${cx + h * 0.9} ${cy - h * 0.1} ${cx + h * 0.7} ${cy + h} ${cx} ${cy + h}C${cx - h * 0.7} ${cy + h} ${cx - h * 0.9} ${cy - h * 0.1} ${cx} ${cy - h}Z`;
}

/** A six-armed snowflake, centred on (cx, cy), of radius `r`. */
function snowflake(cx: number, cy: number, r: number) {
  let d = '';
  for (let arm = 0; arm < 3; arm++) {
    const a = (arm * Math.PI) / 3 + Math.PI / 2;
    const dx = Math.cos(a) * r;
    const dy = Math.sin(a) * r;
    d += `M${cx - dx} ${cy - dy}L${cx + dx} ${cy + dy}`;
  }
  return d;
}

export function IceAndFireThumb() {
  const shapes: ReactElement[] = [];
  rows().forEach((row, y) => {
    [...row].forEach((ch, x) => {
      if (ch === '#') return;
      const x0 = GAP + x * (CELL + GAP);
      const y0 = GAP + y * (CELL + GAP);
      const cx = x0 + CELL / 2;
      const cy = y0 + CELL / 2;
      const red = 'RF123'.includes(ch);
      const blue = 'BIabc'.includes(ch);
      const key = `${x},${y}`;
      shapes.push(
        <rect
          key={`t${key}`}
          x={x0}
          y={y0}
          width={CELL}
          height={CELL}
          fill={red ? RED_TILE : blue ? BLUE_TILE : 'var(--sq)'}
        />,
      );
      if (ch === 'F' || ch === 'I') {
        shapes.push(
          <rect
            key={`h${key}`}
            x={x0}
            y={y0}
            width={CELL}
            height={CELL}
            fill={ch === 'F' ? RED : BLUE}
          />,
        );
        shapes.push(
          ch === 'F' ? (
            <path key={`g${key}`} d={flame(cx, cy, CELL * 0.74)} fill="#fbfbfb" />
          ) : (
            <path
              key={`g${key}`}
              d={snowflake(cx, cy, CELL * 0.36)}
              stroke="#fbfbfb"
              strokeWidth={CELL * 0.12}
              strokeLinecap="round"
            />
          ),
        );
      } else if ('123abc'.includes(ch)) {
        const level = '123'.includes(ch) ? Number(ch) : 'abc'.indexOf(ch) + 1;
        shapes.push(
          <circle
            key={`u${key}`}
            cx={cx}
            cy={cy}
            r={CELL * (0.26 + level * 0.04)}
            fill={red ? RED : BLUE}
            stroke="#fbfbfb"
            strokeWidth={CELL * 0.08}
          />,
        );
      }
    });
  });
  return (
    <svg viewBox="0 0 90 90" role="img" aria-label="A game in progress: fire against ice">
      <rect width="90" height="90" fill="#44515a" />
      {shapes}
    </svg>
  );
}

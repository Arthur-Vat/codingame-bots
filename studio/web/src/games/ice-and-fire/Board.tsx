import './board.css';
import { useGameTheme } from '../../gameThemes';
import {
  boardThemeOf,
  groundName,
  groundUrl,
  mirrored,
  pieceThemeOf,
  pieceUrl,
  sideName,
} from './themes';
import {
  SIZE,
  answerText,
  arrow,
  boardLabel,
  kindName,
  seatName,
  signed,
  toMoveText,
  turnText,
  type Building,
  type IceAndFireFrame,
  type Level,
  type Point,
  type Seat,
  type Unit,
} from './view';

export type { IceAndFireFrame } from './view';

export interface BoardProps {
  frame: IceAndFireFrame;
  /** Accepted and ignored: the game has no human play. */
  interactive?: boolean;
  onAction?: (action: unknown) => void;
  showCoordinates: boolean;
  heat?: Map<string, number> | null;
  highlight?: [number, number] | null;
}

const INDEXES = Array.from({ length: SIZE }, (_, i) => i);
/** The room the coordinates take, left of and above the cells, in cell units. */
const GUTTER = 0.9;
const SEATS: Seat[] = [0, 1];

const side = (seat: Seat): 'r' | 'b' => (seat === 0 ? 'r' : 'b');

/* The mark of a mine spot over the ground of an owned cell that has no mine: a small diamond,
   drawn in 0..1 cell units from the cell's corner. */
const SPOT_PATH = 'M.5 .26L.74 .5L.5 .74L.26 .5Z';
const CROSS_PATH = 'M.3 .3L.7 .7M.7 .3L.3 .7';

/** A piece, drawn at the full cell: its drawing carries its own margins and shadow. */
function Piece({ href }: { href: string }) {
  return <image href={href} x={0} y={0} width={1} height={1} />;
}

function BuildingMark({ building, theme }: { building: Building; theme: string }) {
  const { owner, kind, x, y } = building;
  const name = `${seatName(owner)} ${kindName(kind)} at ${x},${y}`;
  return (
    <g className={`building ${kind} ${side(owner)}`} transform={`translate(${x} ${y})`}>
      <title>{name}</title>
      <Piece href={pieceUrl(theme, `${kind}-${sideName(owner)}`)} />
    </g>
  );
}

function UnitMark({ unit, theme }: { unit: Unit; theme: string }) {
  const { owner, level, x, y } = unit;
  return (
    <g className={`unit ${side(owner)}`} transform={`translate(${x} ${y})`}>
      <title>{`${seatName(owner)} unit level ${level} at ${x},${y}`}</title>
      <Piece href={pieceUrl(theme, `unit-${sideName(owner)}-${level}`)} />
    </g>
  );
}

interface RemovedProps {
  owner: Seat;
  level: Level;
  x: number;
  y: number;
  theme: string;
}

/** A unit removed by the last turn: its piece, faded, under a cross. */
function RemovedMark({ owner, level, x, y, theme }: RemovedProps) {
  return (
    <g className={`removed ${side(owner)}`} transform={`translate(${x} ${y})`}>
      <title>{`${seatName(owner)} unit level ${level} removed at ${x},${y}`}</title>
      <g className="faded">
        <Piece href={pieceUrl(theme, `unit-${sideName(owner)}-${level}`)} />
      </g>
      <path className="cross casing" d={CROSS_PATH} />
      <path className="cross" d={CROSS_PATH} />
    </g>
  );
}

function SeatSummary({ frame, seat }: { frame: IceAndFireFrame; seat: Seat }) {
  return (
    <div className={`iaf-seat ${side(seat)}`}>
      <span className="name">{seatName(seat)}</span>
      <span title="Gold">{frame.gold[seat]} gold</span>
      <span title="Income per turn">{signed(frame.income[seat])}</span>
      <span title="Military power">power {frame.military[seat]}</span>
    </div>
  );
}

interface Tile {
  x: number;
  y: number;
  href: string;
  flip: boolean;
  /** A mine spot that holds no mine. */
  spot: boolean;
  owned: boolean;
}

export default function Board({ frame, showCoordinates }: BoardProps) {
  const chosen = useGameTheme('ice-and-fire');
  // The stored ids are looked up in the lists: an unknown one gives the default.
  const pieces = pieceThemeOf(chosen.pieces).id;
  const ground = boardThemeOf(chosen.board);
  const built = new Set(frame.buildings.map((b) => `${b.x},${b.y}`));
  const spotCells = new Set(frame.mine_spots.map(([x, y]) => `${x},${y}`));
  const tiles: Tile[] = [];
  for (let y = 0; y < SIZE; y++) {
    for (let x = 0; x < SIZE; x++) {
      const owner = frame.owner[y]?.[x] ?? null;
      const mineSpot = spotCells.has(`${x},${y}`);
      const name = groundName({
        void: frame.void[y]?.[x] === true,
        owner,
        active: frame.active[y]?.[x] === true,
        mineSpot,
      });
      tiles.push({
        x,
        y,
        href: groundUrl(ground.id, name),
        flip: ground.mirror && mirrored(x, y),
        spot: mineSpot && !built.has(`${x},${y}`),
        owned: owner !== null,
      });
    }
  }
  const last = frame.last;
  const answer = answerText(frame);
  const toMove = toMoveText(frame);
  const label = boardLabel(frame);

  return (
    <div
      className={`iaf${showCoordinates ? '' : ' coords-off'}`}
      role="group"
      aria-label={label}
      data-pieces={pieces}
      data-board={ground.id}
    >
      <div className="iaf-strip">
        <div className="iaf-turn">
          <span>{turnText(frame)}</span>
          {toMove !== '' && (
            <span className={`to-move${frame.to_move === null ? '' : ` ${side(frame.to_move)}`}`}>
              {toMove}
            </span>
          )}
        </div>
        <div className="iaf-seats">
          {SEATS.map((seat) => (
            <SeatSummary key={seat} frame={frame} seat={seat} />
          ))}
        </div>
        <div className="iaf-answer" title={answer === '' ? undefined : answer}>
          {answer}
        </div>
      </div>
      <svg
        className="iaf-svg"
        viewBox={`${-GUTTER} ${-GUTTER} ${SIZE + GUTTER} ${SIZE + GUTTER}`}
        preserveAspectRatio="xMidYMid meet"
        aria-hidden="true"
      >
        <g className="coords">
          {INDEXES.map((i) => (
            <text key={`x${i}`} className="coord" x={i + 0.5} y={-0.4}>
              {i}
            </text>
          ))}
          {INDEXES.map((i) => (
            <text key={`y${i}`} className="coord" x={-0.45} y={i + 0.5}>
              {i}
            </text>
          ))}
        </g>
        <g className="tiles">
          {tiles.map(({ x, y, href, flip, spot, owned }) => (
            <image
              key={`${x},${y}`}
              className="tile"
              href={href}
              x={x}
              y={y}
              width={1}
              height={1}
              preserveAspectRatio="none"
              transform={flip ? `matrix(-1 0 0 1 ${2 * x + 1} 0)` : undefined}
            >
              {spot && !owned && <title>{`Mine spot at ${x},${y}`}</title>}
            </image>
          ))}
        </g>
        <g className="spots">
          {tiles
            .filter(({ spot, owned }) => spot && owned)
            .map(({ x, y }) => (
              <path
                key={`${x},${y}`}
                className="spot"
                d={SPOT_PATH}
                transform={`translate(${x} ${y})`}
              >
                <title>{`Mine spot at ${x},${y}`}</title>
              </path>
            ))}
        </g>
        <g className="buildings">
          {frame.buildings.map((building) => (
            <BuildingMark key={`${building.x},${building.y}`} building={building} theme={pieces} />
          ))}
        </g>
        {last !== null && (
          <g className="removed-units">
            {last.removed.map((r) => (
              <RemovedMark
                key={r.id}
                owner={r.owner}
                level={r.level}
                x={r.x}
                y={r.y}
                theme={pieces}
              />
            ))}
          </g>
        )}
        <g className="units">
          {frame.units.map((unit) => (
            <UnitMark key={unit.id} unit={unit} theme={pieces} />
          ))}
        </g>
        {last !== null && (
          <g className="last">
            {last.built.map(({ x, y, kind }) => (
              <rect
                key={`b${x},${y}`}
                className="built"
                x={x + 0.06}
                y={y + 0.06}
                width={0.88}
                height={0.88}
              >
                <title>{`Built ${kind} at ${x},${y}`}</title>
              </rect>
            ))}
            {last.trained.map(({ id, x, y, level }) => (
              <circle key={`t${id}`} className="trained" cx={x + 0.5} cy={y + 0.5} r={0.43}>
                <title>{`Trained level ${level} unit at ${x},${y}`}</title>
              </circle>
            ))}
            {last.moved.map(({ id, from, to }) => (
              <MoveArrow key={`m${id}`} from={from} to={to} />
            ))}
          </g>
        )}
      </svg>
    </div>
  );
}

function MoveArrow({ from, to }: { from: Point; to: Point }) {
  const a = arrow(from, to);
  if (a === null) return null;
  return (
    <g className="move">
      <title>{`Moved from ${from[0]},${from[1]} to ${to[0]},${to[1]}`}</title>
      <line className="casing" x1={a.x1} y1={a.y1} x2={a.x2} y2={a.y2} />
      <polygon className="casing" points={a.head} />
      <line className="shaft" x1={a.x1} y1={a.y1} x2={a.x2} y2={a.y2} />
      <polygon className="tip" points={a.head} />
    </g>
  );
}

import './board.css';
import {
  SIZE,
  answerText,
  arrow,
  boardLabel,
  hatchPath,
  inactiveCells,
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

/* A building spans a cell, drawn in 0..1 cell units from the cell's corner. */
const HQ_PATH = 'M.15 .85V.25H.32V.38H.43V.25H.57V.38H.68V.25H.85V.85Z';
const MINE_PATH = 'M.5 .1L.9 .5L.5 .9L.1 .5Z';
const MINE_CORE = 'M.5 .32L.68 .5L.5 .68L.32 .5Z';
const TOWER_PATH = 'M.5 .1L.9 .88H.1Z';
const SPOT_PATH = 'M.5 .26L.74 .5L.5 .74L.26 .5Z';

function BuildingMark({ building }: { building: Building }) {
  const { owner, kind, x, y } = building;
  const name = `${seatName(owner)} ${kindName(kind)} at ${x},${y}`;
  return (
    <g className={`building ${kind} ${side(owner)}`} transform={`translate(${x} ${y})`}>
      <title>{name}</title>
      {kind === 'hq' && (
        <>
          <path className="shape" d={HQ_PATH} />
          <text className="letter" x={0.5} y={0.7}>
            {owner === 0 ? 'R' : 'B'}
          </text>
        </>
      )}
      {kind === 'mine' && (
        <>
          <path className="shape" d={MINE_PATH} />
          <path className="gold" d={MINE_CORE} />
        </>
      )}
      {kind === 'tower' && <path className="shape" d={TOWER_PATH} />}
    </g>
  );
}

function UnitMark({ unit }: { unit: Unit }) {
  const { owner, level, x, y } = unit;
  return (
    <g className={`unit ${side(owner)}`} transform={`translate(${x} ${y})`}>
      <title>{`${seatName(owner)} unit level ${level} at ${x},${y}`}</title>
      {owner === 1 && <circle className="halo" cx={0.5} cy={0.5} r={0.42} />}
      <circle className="disc" cx={0.5} cy={0.5} r={0.32} />
      <text className="level" x={0.5} y={0.52}>
        {level}
      </text>
    </g>
  );
}

function RemovedMark({ owner, level, x, y }: { owner: Seat; level: Level; x: number; y: number }) {
  return (
    <g className={`removed ${side(owner)}`} transform={`translate(${x} ${y})`}>
      <title>{`${seatName(owner)} unit level ${level} removed at ${x},${y}`}</title>
      <circle className="disc" cx={0.5} cy={0.5} r={0.32} />
      <path className="cross" d="M.34 .34L.66 .66M.66 .34L.34 .66" />
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

export default function Board({ frame, showCoordinates }: BoardProps) {
  const tiles: { x: number; y: number; className: string }[] = [];
  for (let y = 0; y < SIZE; y++) {
    for (let x = 0; x < SIZE; x++) {
      if (frame.void[y]?.[x] === true) continue;
      const owner = frame.owner[y]?.[x] ?? null;
      const active = frame.active[y]?.[x] === true;
      const className =
        owner === null ? 'tile neutral' : `tile ${side(owner)}${active ? '' : ' off'}`;
      tiles.push({ x, y, className });
    }
  }
  const built = new Set(frame.buildings.map((b) => `${b.x},${b.y}`));
  const spots = frame.mine_spots.filter(([x, y]) => !built.has(`${x},${y}`));
  const last = frame.last;
  const answer = answerText(frame);
  const toMove = toMoveText(frame);
  const label = boardLabel(frame);

  return (
    <div className={`iaf${showCoordinates ? '' : ' coords-off'}`} role="group" aria-label={label}>
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
          {tiles.map(({ x, y, className }) => (
            <rect
              key={`${x},${y}`}
              className={className}
              x={x + 0.03}
              y={y + 0.03}
              width={0.94}
              height={0.94}
            />
          ))}
        </g>
        <path className="hatch r" d={hatchPath(inactiveCells(frame, 0))} />
        <path className="hatch b" d={hatchPath(inactiveCells(frame, 1))} />
        <g className="spots">
          {spots.map(([x, y]) => (
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
            <BuildingMark key={`${building.x},${building.y}`} building={building} />
          ))}
        </g>
        {last !== null && (
          <g className="removed-units">
            {last.removed.map((r) => (
              <RemovedMark key={r.id} owner={r.owner} level={r.level} x={r.x} y={r.y} />
            ))}
          </g>
        )}
        <g className="units">
          {frame.units.map((unit) => (
            <UnitMark key={unit.id} unit={unit} />
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

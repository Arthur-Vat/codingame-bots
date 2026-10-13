import { useMemo, useState, type ReactNode } from 'react';
import {
  getHistoryRecord,
  getRecord,
  postHistory,
  postHistoryText,
  saveSession,
  type GameSummary,
} from './api';
import { heatShares, selectLines } from './analysisView';
import {
  readEngineOn,
  readHeat,
  readLines,
  writeEngineOn,
  writeHeat,
  writeLines,
  type LineCount,
} from './analysisSettings';
import {
  exportName,
  formatDate,
  lastFrame,
  type AnalysisModel,
  type AnalysisPlayer,
} from './analysis';
import { arenaRecordText } from './arena';
import { BoardSlot } from './BoardSlot';
import { download, downloadText, errorMessage } from './browser';
import { emptyFrameFor, moveCellFor, renderers, resultDetailFor } from './games/registry';
import { EnginePanel } from './EnginePanel';
import { EvalBar } from './EvalBar';
import { EvalChart } from './EvalChart';
import { EndBlock, Row, useKeepInView } from './GameTable';
import { SeatMark } from './Marks';
import { recallFile } from './loadedFiles';
import { frameAfterTurn, timeUsed, turnOfFrame } from './play';
import { PlaybackBar } from './PlaybackBar';
import { routeHash, type AnalysisKind } from './route';
import { formatSpent } from './clock';
import { moveRows, resultText, seatMark } from './sessionView';
import { Toast, useToast } from './Toast';
import { Redirect, StatusPanel } from './Panels';
import { usePlayback } from './usePlayback';
import { useAnalysis } from './useAnalysis';
import { useEngine } from './useEngine';

/** One player's strip: mark, name, how it played, and the time its answers took. */
function AnalysisStrip({ model, seat }: { model: AnalysisModel; seat: number }) {
  const player: AnalysisPlayer | undefined = model.players[seat];
  const spent = timeUsed(model, seat);
  return (
    <div className="player" data-testid={`player-${seat}`}>
      <div className="who">
        <SeatMark seat={seat} className="chip" />
        <b>{player?.name ?? `Player ${seat + 1}`}</b>
        <small>{seatMark(seat)}</small>
      </div>
      {player?.kind === 'bot' && (
        <span className="clock spent" title="Time the bot used">
          {formatSpent(spent)}
        </span>
      )}
    </div>
  );
}

function playerLine(player: AnalysisPlayer): string {
  return player.settings === null ? player.name : `${player.name}, ${player.settings}`;
}

function InfoCard({ model, resultLine }: { model: AnalysisModel; resultLine: string }) {
  return (
    <section className="panel info">
      <h3>Analysis board</h3>
      <dl>
        <Row term="Source" value={model.source} />
        <Row term="Date" value={formatDate(model.unixTime)} />
        <Row term="Seed" value={String(model.seed)} />
        <Row term="X" value={playerLine(model.players[0])} />
        <Row term="O" value={playerLine(model.players[1])} />
        <Row term="Result" value={resultLine} />
      </dl>
    </section>
  );
}

function AnalysisMoves({
  model,
  shownTurn,
  atLatest,
  headline,
  detail,
  onPick,
}: {
  model: AnalysisModel;
  shownTurn: number;
  atLatest: boolean;
  headline: string;
  detail: string;
  onPick: (turn: number) => void;
}) {
  const list = useKeepInView(shownTurn, model.turns.length, true, atLatest);
  const cell = (move: { turn: number; text: string }) => (
    <button
      type="button"
      className={move.turn === shownTurn ? 'cur' : undefined}
      aria-current={move.turn === shownTurn ? 'true' : undefined}
      onClick={() => onPick(move.turn)}
    >
      {move.text}
    </button>
  );
  return (
    <div className="moves" aria-label="Moves" ref={list}>
      {moveRows(model.turns).map((row) => (
        <MoveRow key={row.number} number={row.number}>
          {cell(row.moves[0])}
          {row.moves[1] === null ? <span /> : cell(row.moves[1])}
        </MoveRow>
      ))}
      <div className="result" data-testid="result">
        <b>{headline}</b>
        {detail !== '' && <span>{detail}</span>}
      </div>
    </div>
  );
}

function MoveRow({ number, children }: { number: number; children: ReactNode }) {
  return (
    <>
      <span className="n">{number}</span>
      {children}
    </>
  );
}

/** The record to export, whatever the game came from but an arena game (kept as text). */
async function recordOf(model: AnalysisModel): Promise<unknown> {
  if (model.kind === 'saved') return getHistoryRecord(model.id);
  if (model.kind === 'session') return getRecord(model.id);
  if (model.kind === 'arena') throw new Error('An arena game is kept as text.');
  const file = recallFile(model.id);
  if (file === null) throw new Error('This file is no longer loaded.');
  return file.record;
}

/** Where the way back leads, and what it says. */
export interface BackLink {
  hash: string;
  label: string;
}

/** What a game just played offers from the analysis board; it acts through the Play tab. */
export interface AfterActions {
  onRematch: () => void;
  onNewGame: () => void;
}

function Back({ back }: { back: BackLink }) {
  return (
    <a className="back-link" href={back.hash}>
      {back.label}
    </a>
  );
}

function AnalysisScreen({
  model,
  game,
  back,
  actions,
}: {
  model: AnalysisModel;
  /** The game's entry in the server's list (null until it is loaded): its releases and whether it has analysis. */
  game: GameSummary | null;
  back: BackLink;
  actions: AfterActions | null;
}) {
  const toast = useToast();
  const [coordinates, setCoordinates] = useState(false);
  const [engineOn, setEngineOn] = useState(readEngineOn);
  const [heatOn, setHeatOn] = useState(readHeat);
  const [lineCount, setLineCount] = useState<LineCount>(readLines);
  // The first move of the line the pointer is over.
  const [hovered, setHovered] = useState<string | null>(null);
  const [savedHere, setSavedHere] = useState(false);
  const last = lastFrame(model);
  // The analysis board opens on the first frame.
  const playback = usePlayback(last, 0, `${model.kind}/${model.id}`, { active: true, keys: true });
  const cursor = playback.cursor;
  const goto = playback.goto;
  const toggle = playback.toggle;

  const available = game?.analysis === true;
  const engine = useEngine(model, cursor, available, engineOn);
  const report = engine.live.report;
  const shownLines = useMemo(() => selectLines(report, lineCount), [report, lineCount]);
  const heat = useMemo(
    () =>
      available && engineOn && heatOn
        ? heatShares(report, (text) => moveCellFor(model.game, text))
        : null,
    [available, engineOn, heatOn, report, model.game],
  );
  // A line that is no longer shown outlines nothing, whatever the pointer did.
  const highlight =
    hovered !== null && shownLines.some((line) => line.move === hovered)
      ? (moveCellFor(model.game, hovered)?.at ?? null)
      : null;
  const release = game?.releases[0];
  const engineName = release === undefined ? 'Engine' : `${release} engine`;

  const Renderer = renderers[model.game];
  const frame = model.frames[cursor] ?? emptyFrameFor(model.game);
  const finalFrame = model.frames[last];
  const detail =
    model.result.end.kind === 'finished' && finalFrame !== undefined
      ? resultDetailFor(model.game, finalFrame)
      : null;
  const text = resultText(model.result, model.players, detail);
  const endDetail = model.hiddenTurns > 0 ? `${text.detail}, not shown` : text.detail;
  const resultLine = `${text.headline}${endDetail ? `, ${endDetail}` : ''}`;

  const shownTurn = turnOfFrame(cursor, model.openingTurns);
  const saved = model.saved || savedHere;

  const save = async () => {
    try {
      // An arena game is saved as the text it was published in.
      const { duplicate } =
        model.kind === 'arena'
          ? await postHistoryText(await arenaRecordText(model.id))
          : model.kind === 'session'
            ? await saveSession(model.id)
            : await postHistory(await recordOf(model));
      setSavedHere(true);
      toast.show(duplicate ? 'Already in your history' : 'Saved to your history');
    } catch (failure) {
      toast.show(errorMessage(failure));
    }
  };
  const exportFile = async () => {
    try {
      if (model.kind === 'arena') {
        downloadText(exportName(model), await arenaRecordText(model.id));
      } else {
        download(exportName(model), await recordOf(model));
      }
    } catch (failure) {
      toast.show(errorMessage(failure));
    }
  };

  return (
    <>
      <main className="wrap game">
        <aside className="side side-left">
          <Back back={back} />
          <InfoCard model={model} resultLine={resultLine} />
          <section className="panel toggles" aria-label="Board settings">
            <span className="label">Board</span>
            <label className="switch">
              Show coordinates
              <input
                type="checkbox"
                checked={coordinates}
                onChange={(event) => setCoordinates(event.target.checked)}
              />
            </label>
            {available && (
              <label className="switch">
                Heatmap of best moves
                <input
                  type="checkbox"
                  checked={heatOn}
                  disabled={!engineOn}
                  onChange={(event) => {
                    setHeatOn(event.target.checked);
                    writeHeat(event.target.checked);
                  }}
                />
              </label>
            )}
            <p className="hint">Keys: ← → step, Home/End jump, Space plays.</p>
          </section>
        </aside>

        <section className="board-col">
          <div className="board-row">
            <BoardSlot
              showCoordinates={coordinates}
              label={`${model.game} board`}
              bare={Renderer !== undefined}
            >
              {Renderer !== undefined && frame !== undefined ? (
                <Renderer
                  frame={frame}
                  interactive={false}
                  showCoordinates={coordinates}
                  heat={heat}
                  highlight={highlight}
                />
              ) : undefined}
            </BoardSlot>
            <EvalBar report={available && engineOn ? report : null} />
          </div>
          <PlaybackBar
            cursor={cursor}
            last={last}
            playing={playback.playing}
            speed={playback.speed}
            onGoto={goto}
            onToggle={toggle}
            onSpeed={playback.setSpeed}
          />
          {engine.chart !== null && (
            <div className="chart-row">
              <EvalChart values={engine.chart.values} cursor={cursor} onGoto={goto} />
            </div>
          )}
        </section>

        <aside className="side side-right">
          {available && (
            <EnginePanel
              name={engineName}
              on={engineOn}
              onToggle={(on) => {
                setEngineOn(on);
                writeEngineOn(on);
              }}
              live={engine.live}
              problem={engine.chart?.error ?? null}
              lines={lineCount}
              onLines={(count) => {
                setLineCount(count);
                writeLines(count);
              }}
              ply={model.openingTurns + cursor}
              onHover={setHovered}
              onKeep={engine.keep}
            />
          )}
          <section className="panel table">
            <AnalysisStrip model={model} seat={1} />
            <AnalysisMoves
              model={model}
              shownTurn={shownTurn}
              atLatest={cursor >= last}
              headline={text.headline}
              detail={endDetail}
              onPick={(turn) => goto(frameAfterTurn(turn, model.openingTurns))}
            />
            <div className="controls">
              <button
                className="ctrl"
                type="button"
                disabled={saved || model.turns.length === 0}
                onClick={() => void save()}
              >
                {saved ? 'In your history' : 'Save to history'}
              </button>
              <button
                className="ctrl"
                type="button"
                disabled={model.turns.length === 0}
                onClick={() => void exportFile()}
              >
                Export file
              </button>
            </div>
            {actions !== null && (
              <EndBlock
                testId="analysis-actions"
                disabled={false}
                onRematch={actions.onRematch}
                onNewGame={actions.onNewGame}
              />
            )}
            <AnalysisStrip model={model} seat={0} />
          </section>
        </aside>
      </main>
      <Toast text={toast.text} />
    </>
  );
}

/** The game an analysis board route names, loaded; its page when the game is loaded. */
function AnalysisLoader({
  gameId,
  game,
  kind,
  id,
  back,
  actions,
}: {
  gameId: string;
  game: GameSummary | null;
  kind: AnalysisKind;
  id: string;
  back: BackLink;
  actions: AfterActions | null;
}) {
  const { model, error, playing } = useAnalysis(kind, id);
  if (model === null) {
    if (playing) {
      return (
        <StatusPanel text="This game is still being played.">
          <Back back={back} />
        </StatusPanel>
      );
    }
    return error === null ? (
      <StatusPanel text="Loading the game…" />
    ) : (
      <StatusPanel text={error} alert>
        <Back back={back} />
      </StatusPanel>
    );
  }
  // A hand-typed address with another game's id: the game belongs to its own game's page.
  if (model.game !== gameId) {
    return (
      <Redirect
        hash={routeHash({
          page: 'game',
          gameId: model.game,
          tab: 'analysis',
          board: { kind, id },
        })}
      />
    );
  }
  // A new game starts from its first frame, with its own playback.
  return (
    <AnalysisScreen
      key={`${model.kind}/${model.id}`}
      model={model}
      game={game}
      back={back}
      actions={actions}
    />
  );
}

/**
 * The analysis board of a game: a game of the history, a file or a game just played, with
 * playback, Save and Export. `back` leads to where the game came from; `actions` (Rematch and New
 * game) is given only for a game the Play tab still holds.
 */
export function AnalysisTab({
  gameId,
  game,
  board,
  back,
  actions,
}: {
  gameId: string;
  game: GameSummary | null;
  board: { kind: AnalysisKind; id: string };
  back: BackLink;
  actions: AfterActions | null;
}) {
  return (
    <AnalysisLoader
      gameId={gameId}
      game={game}
      kind={board.kind}
      id={board.id}
      back={back}
      actions={actions}
    />
  );
}

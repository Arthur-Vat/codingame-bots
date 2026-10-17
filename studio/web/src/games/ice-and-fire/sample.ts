/* A frame for the tests: a small game in progress, as the adapter writes it. */
import { emptyFrame, type IceAndFireFrame } from './view';

/** A new frame each call. */
export function sample(): IceAndFireFrame {
  const frame = emptyFrame();
  frame.void[0]![11] = true;
  frame.owner[0]![0] = 0;
  frame.active[0]![0] = true;
  frame.owner[0]![1] = 0;
  frame.active[0]![1] = true;
  frame.owner[2]![0] = 0; // cut off
  frame.owner[11]![11] = 1;
  frame.active[11]![11] = true;
  frame.mine_spots = [
    [3, 3],
    [8, 8],
  ];
  frame.buildings = [
    { owner: 0, kind: 'hq', x: 0, y: 0 },
    { owner: 1, kind: 'hq', x: 11, y: 11 },
  ];
  frame.units = [
    { id: 1, owner: 0, level: 1, x: 1, y: 0 },
    { id: 2, owner: 1, level: 3, x: 11, y: 10 },
  ];
  frame.gold = [12, 7];
  frame.income = [5, -2];
  frame.military = [1, 3];
  frame.turn = 4;
  frame.to_move = 1;
  frame.last = {
    seat: 0,
    answer: 'TRAIN 1 1 0;MOVE 1 1 0',
    moved: [{ id: 1, from: [0, 0], to: [1, 0] }],
    trained: [{ id: 3, level: 1, x: 0, y: 1 }],
    built: [{ kind: 'mine', x: 3, y: 3 }],
    removed: [{ id: 9, owner: 1, level: 2, x: 5, y: 5 }],
  };
  return frame;
}

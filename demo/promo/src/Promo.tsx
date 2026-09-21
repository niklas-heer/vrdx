import { Sequence } from "remotion";
import { Intro } from "./scenes/Intro";
import { Problem } from "./scenes/Problem";
import { Record } from "./scenes/Record";
import { Cli } from "./scenes/Cli";
import { Graph } from "./scenes/Graph";
import { Agents } from "./scenes/Agents";
import { Outro } from "./scenes/Outro";

export const FPS = 30;

const scenes = [
  { Component: Intro, frames: 130 },
  { Component: Problem, frames: 200 },
  { Component: Record, frames: 260 },
  { Component: Cli, frames: 260 },
  { Component: Graph, frames: 250 },
  { Component: Agents, frames: 230 },
  { Component: Outro, frames: 170 },
];

export const PROMO_DURATION = scenes.reduce((sum, s) => sum + s.frames, 0);

export const Promo = () => {
  let from = 0;
  return (
    <>
      {scenes.map(({ Component, frames }, i) => {
        const start = from;
        from += frames;
        return (
          <Sequence key={i} from={start} durationInFrames={frames} name={Component.name}>
            <Component durationInFrames={frames} />
          </Sequence>
        );
      })}
    </>
  );
};

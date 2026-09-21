import { AbsoluteFill, useCurrentFrame } from "remotion";
import { Eyebrow, Headline, Rise, Scene, Terminal, Typed, useSceneFade } from "../components";
import { colors } from "../theme";

const P = () => <span style={{ color: colors.green, fontWeight: 700 }}>❯ </span>;

const Out = ({ delay, children }: { delay: number; children: React.ReactNode }) => {
  const frame = useCurrentFrame();
  return <span style={{ opacity: frame >= delay ? 1 : 0 }}>{children}</span>;
};

export const Cli = ({ durationInFrames }: { durationInFrames: number }) => (
  <Scene style={{ opacity: useSceneFade(durationInFrames) }}>
    <AbsoluteFill style={{ padding: "100px 140px" }}>
      <Eyebrow>One native binary</Eyebrow>
      <Headline size={64}>Find, inspect, connect.</Headline>
      <div style={{ display: "flex", gap: 50, marginTop: 50, alignItems: "flex-start" }}>
        <Terminal title="orders-api — bash" style={{ width: 1080, minHeight: 600 }}>
          <P />
          <Typed text="vrdx chain 01JNG" delay={10} speed={1.6} />
          {"\n"}
          <Out delay={48}>
            <span style={{ color: colors.superseded }}>2025-03-04 [superseded]</span> Store sessions in Redis{"\n"}
            <span style={{ color: colors.muted }}>  ID: 01JNGADM503XR7Q2WKH9TB5FMV</span>{"\n"}
            {"\n"}
            <span style={{ color: colors.accepted }}>2026-01-15 [accepted]</span> Sign sessions into cookies{"\n"}
            <span style={{ color: colors.muted }}>  ID: 01KF179SW0Q7ZH4NDM8XK2TR6V</span>{"\n"}
            {"\n"}
          </Out>
          <Out delay={70}>
            <P />
          </Out>
          <Typed
            text={`vrdx context "How do we handle sessions?" --json | jq -c '.data.decisions[] | {title, status, applies}'`}
            delay={78}
            speed={0.6}
          />
          {"\n"}
          <Out delay={150}>
            <span style={{ color: colors.proposed }}>{`{"title":"Store sessions in Redis","status":"superseded","applies":false}`}</span>{"\n"}
            <span style={{ color: colors.proposed }}>{`{"title":"Use Postgres for orders","status":"accepted","applies":true}`}</span>{"\n"}
            <span style={{ color: colors.proposed }}>{`{"title":"Sign sessions into cookies","status":"accepted","applies":true}`}</span>{"\n"}
            <span style={{ color: colors.proposed }}>{`{"title":"Adopt OpenTelemetry tracing","status":"proposed","applies":false}`}</span>
          </Out>
        </Terminal>
        <div style={{ display: "flex", flexDirection: "column", gap: 30, width: 500, paddingTop: 20 }}>
          {[
            [20, "list · search · show", "Browse and filter by status and tag."],
            [60, "relations · chain", "Follow every replacement to the current decision."],
            [120, "context · suggest", "Ranked, connected evidence for an AI, with source paths."],
            [170, "validate · fmt · rebuild", "Deterministic checks and a stable JSON envelope."],
          ].map(([at, cmd, text]) => (
            <Rise key={cmd as string} delay={at as number}>
              <div style={{ fontFamily: "monospace", color: colors.green, fontSize: 26 }}>{cmd}</div>
              <div style={{ fontSize: 27, color: colors.ink, lineHeight: 1.35 }}>{text}</div>
            </Rise>
          ))}
        </div>
      </div>
    </AbsoluteFill>
  </Scene>
);

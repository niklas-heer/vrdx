import { AbsoluteFill, useCurrentFrame } from "remotion";
import { Card, Eyebrow, Headline, Rise, Scene, Typed, useSceneFade } from "../components";
import { colors, fonts } from "../theme";

const lines: Array<[string, string]> = [
  ["+++", colors.muted],
  ['schema_version = 1', colors.ink],
  ['id = "01KF179SW0Q7ZH4NDM8XK2TR6V"', colors.superseded],
  ['title = "Sign sessions into cookies"', colors.ink],
  ['date = "2026-01-15"', colors.ink],
  ['status = "accepted"', colors.accepted],
  ['tags = ["sessions", "security"]', colors.ink],
  ['supersedes = ["01JNGADM503XR7Q2WKH9TB5FMV"]', colors.edgeSupersedes],
  ["+++", colors.muted],
  ["", colors.ink],
  ["## Decision", colors.proposed],
  ["", colors.ink],
  ["Carry the session in a signed, HttpOnly cookie", colors.ink],
  ["and drop the Redis session store.", colors.ink],
  ["", colors.ink],
  ["## Why", colors.proposed],
  ["", colors.ink],
  ["Two outages traced back to Redis evicting", colors.ink],
  ["sessions under memory pressure. …", colors.ink],
];

const callouts = [
  { at: 70, text: "A stable ULID lives in the metadata. Rename the file freely." },
  { at: 110, text: "Five lifecycle states. Only accepted records apply." },
  { at: 150, text: "One field links the replacement; vrdx derives the inverse." },
  { at: 190, text: "The body is plain Markdown: decision, why, consequences." },
];

export const Record = ({ durationInFrames }: { durationInFrames: number }) => {
  const frame = useCurrentFrame();
  return (
    <Scene style={{ opacity: useSceneFade(durationInFrames) }}>
      <AbsoluteFill style={{ padding: "100px 140px" }}>
        <Eyebrow>Markdown is the source</Eyebrow>
        <Headline size={64}>A decision is an ordinary file.</Headline>
        <div style={{ display: "flex", gap: 70, marginTop: 50 }}>
          <Card style={{ width: 1000, padding: "30px 36px", fontFamily: fonts.mono, fontSize: 26, lineHeight: 1.45 }}>
            {lines.map(([text, color], i) => (
              <div key={i} style={{ color, minHeight: 38 }}>
                <Typed text={text} delay={6 + i * 9} speed={0.55} />
              </div>
            ))}
          </Card>
          <div style={{ display: "flex", flexDirection: "column", gap: 26, paddingTop: 10, width: 560 }}>
            {callouts.map((c) => (
              <Rise key={c.at} delay={c.at}>
                <div style={{ display: "flex", gap: 18, alignItems: "flex-start" }}>
                  <span
                    style={{
                      marginTop: 12,
                      width: 12,
                      height: 12,
                      borderRadius: 999,
                      background: frame >= c.at ? colors.green : colors.line,
                      flexShrink: 0,
                    }}
                  />
                  <div style={{ fontSize: 30, lineHeight: 1.35 }}>{c.text}</div>
                </div>
              </Rise>
            ))}
          </div>
        </div>
      </AbsoluteFill>
    </Scene>
  );
};

import { AbsoluteFill, interpolate, useCurrentFrame } from "remotion";
import { Card, Eyebrow, Headline, Rise, Scene, useSceneFade } from "../components";
import { colors, fonts } from "../theme";

const chatter = [
  { where: "#backend · Slack", text: "so are we dropping redis for sessions or not?", at: 10 },
  { where: "PR #412 review", text: "I thought we agreed on cookies last quarter?", at: 24 },
  { where: "Meeting notes, Jan 15", text: "Decision: signed cookies. (see thread)", at: 38 },
];

export const Problem = ({ durationInFrames }: { durationInFrames: number }) => {
  const frame = useCurrentFrame();
  const fade = interpolate(frame, [80, 120], [1, 0.18], { extrapolateLeft: "clamp", extrapolateRight: "clamp" });
  const fileIn = interpolate(frame, [90, 120], [0, 1], { extrapolateLeft: "clamp", extrapolateRight: "clamp" });
  return (
    <Scene style={{ opacity: useSceneFade(durationInFrames) }}>
      <AbsoluteFill style={{ padding: "120px 140px" }}>
        <Eyebrow>The problem</Eyebrow>
        <Headline size={72}>
          Decisions outlive the <br /> conversations that produced them.
        </Headline>
        <div style={{ display: "flex", gap: 60, marginTop: 70, alignItems: "flex-start" }}>
          <div style={{ display: "flex", flexDirection: "column", gap: 22, width: 900, opacity: fade }}>
            {chatter.map((c) => (
              <Rise key={c.where} delay={c.at}>
                <Card style={{ padding: "22px 28px", boxShadow: "none" }}>
                  <div style={{ color: colors.muted, fontSize: 20, marginBottom: 6 }}>{c.where}</div>
                  <div style={{ fontSize: 28 }}>{c.text}</div>
                </Card>
              </Rise>
            ))}
          </div>
          <div style={{ opacity: fileIn, transform: `translateY(${(1 - fileIn) * 30}px)`, width: 640 }}>
            <Card style={{ padding: "26px 30px", borderColor: colors.green }}>
              <div style={{ fontFamily: fonts.mono, fontSize: 20, color: colors.green, marginBottom: 14 }}>
                decisions/2026-01-15_162000000_sign-sessions-into-cookies.md
              </div>
              <div style={{ fontFamily: fonts.mono, fontSize: 24, lineHeight: 1.5, color: colors.ink }}>
                status = "accepted"{"\n"}
                <br />
                supersedes = ["01JNGADM50…"]
              </div>
              <div style={{ marginTop: 18, fontSize: 24, color: colors.muted }}>
                One file keeps the choice, the reasoning and what it replaced.
              </div>
            </Card>
          </div>
        </div>
      </AbsoluteFill>
    </Scene>
  );
};

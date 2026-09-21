import { AbsoluteFill, Img, interpolate, spring, staticFile, useCurrentFrame, useVideoConfig } from "remotion";
import { Rise, Scene, useSceneFade } from "../components";
import { colors, fonts } from "../theme";

export const Intro = ({ durationInFrames }: { durationInFrames: number }) => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();
  const pop = spring({ frame, fps, config: { damping: 14, stiffness: 90, mass: 0.9 } });
  const glow = interpolate(frame, [0, 40], [0, 1], { extrapolateRight: "clamp" });
  return (
    <Scene style={{ opacity: useSceneFade(durationInFrames) }}>
      <AbsoluteFill style={{ alignItems: "center", justifyContent: "center", gap: 26 }}>
        <div
          style={{
            position: "absolute",
            width: 900,
            height: 900,
            borderRadius: 999,
            background: `radial-gradient(circle, rgba(156,214,181,${0.16 * glow}) 0%, transparent 60%)`,
          }}
        />
        <div style={{ display: "flex", alignItems: "center", gap: 34, transform: `scale(${pop})` }}>
          <Img src={staticFile("logo.svg")} style={{ width: 150, height: 150 }} />
          <div style={{ fontFamily: fonts.serif, fontSize: 150, letterSpacing: "-0.03em", lineHeight: 1 }}>
            vrdx<span style={{ color: colors.green }}>.</span>
          </div>
        </div>
        <Rise delay={22}>
          <div style={{ fontSize: 46, color: colors.ink, fontFamily: fonts.serif }}>
            Keep the decision. <span style={{ color: colors.green }}>Keep the why.</span>
          </div>
        </Rise>
        <Rise delay={40}>
          <div style={{ fontSize: 26, color: colors.muted, letterSpacing: "0.02em" }}>
            Engineering decisions in Markdown. A Rust CLI for people and AI.
          </div>
        </Rise>
      </AbsoluteFill>
    </Scene>
  );
};

import { AbsoluteFill, Img, staticFile } from "remotion";
import { Rise, Scene, Typed, useSceneFade } from "../components";
import { colors, fonts } from "../theme";

export const Outro = ({ durationInFrames }: { durationInFrames: number }) => (
  <Scene style={{ opacity: useSceneFade(durationInFrames) }}>
    <AbsoluteFill style={{ alignItems: "center", justifyContent: "center", gap: 30 }}>
      <Rise>
        <div style={{ display: "flex", alignItems: "center", gap: 26 }}>
          <Img src={staticFile("logo.svg")} style={{ width: 110, height: 110 }} />
          <div style={{ fontFamily: fonts.serif, fontSize: 110, letterSpacing: "-0.03em" }}>
            vrdx<span style={{ color: colors.green }}>.</span>
          </div>
        </div>
      </Rise>
      <Rise delay={14}>
        <div
          style={{
            fontFamily: fonts.mono,
            fontSize: 40,
            padding: "22px 40px",
            borderRadius: 16,
            background: colors.sidebar,
            border: `1px solid ${colors.line}`,
          }}
        >
          <span style={{ color: colors.green }}>$ </span>
          <Typed text="brew install niklas-heer/tap/vrdx" delay={26} speed={1.3} cursor />
        </div>
      </Rise>
      <Rise delay={70}>
        <div style={{ fontSize: 30, color: colors.muted, textAlign: "center", lineHeight: 1.5 }}>
          No database, account or external AI service. One native Rust binary.
          <br />
          <span style={{ color: colors.ink }}>github.com/niklas-heer/vrdx</span> · MIT
        </div>
      </Rise>
    </AbsoluteFill>
  </Scene>
);

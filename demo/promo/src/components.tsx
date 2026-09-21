import type { CSSProperties, ReactNode } from "react";
import { AbsoluteFill, interpolate, spring, useCurrentFrame, useVideoConfig } from "remotion";
import { colors, fonts } from "./theme";

export const Scene = ({ children, style }: { children: ReactNode; style?: CSSProperties }) => (
  <AbsoluteFill
    style={{
      backgroundColor: colors.paper,
      backgroundImage: `radial-gradient(${colors.line} 1px, transparent 1px)`,
      backgroundSize: "28px 28px",
      color: colors.ink,
      fontFamily: fonts.sans,
      ...style,
    }}
  >
    {children}
  </AbsoluteFill>
);

/** Fades and slides children up, starting at `delay` frames into the sequence. */
export const Rise = ({
  children,
  delay = 0,
  style,
  distance = 28,
}: {
  children: ReactNode;
  delay?: number;
  style?: CSSProperties;
  distance?: number;
}) => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();
  const t = spring({ frame: frame - delay, fps, config: { damping: 200, stiffness: 120 } });
  return (
    <div style={{ opacity: t, transform: `translateY(${(1 - t) * distance}px)`, ...style }}>
      {children}
    </div>
  );
};

/** Renders a string progressively, one character per `speed` frames. */
export const Typed = ({
  text,
  delay = 0,
  speed = 1.4,
  cursor = false,
  style,
}: {
  text: string;
  delay?: number;
  speed?: number;
  cursor?: boolean;
  style?: CSSProperties;
}) => {
  const frame = useCurrentFrame();
  const shown = Math.max(0, Math.min(text.length, Math.floor((frame - delay) / speed)));
  const done = shown >= text.length;
  const blink = Math.floor(frame / 15) % 2 === 0;
  return (
    <span style={style}>
      {text.slice(0, shown)}
      {cursor && (!done || blink) && frame >= delay ? (
        <span style={{ background: colors.green, color: colors.green, marginLeft: 2 }}>_</span>
      ) : null}
    </span>
  );
};

export const Eyebrow = ({ children, delay = 0 }: { children: ReactNode; delay?: number }) => (
  <Rise delay={delay}>
    <div
      style={{
        fontSize: 22,
        letterSpacing: "0.28em",
        textTransform: "uppercase",
        color: colors.green,
        fontWeight: 600,
      }}
    >
      {children}
    </div>
  </Rise>
);

export const Headline = ({
  children,
  delay = 0,
  size = 84,
}: {
  children: ReactNode;
  delay?: number;
  size?: number;
}) => (
  <Rise delay={delay}>
    <h1
      style={{
        fontFamily: fonts.serif,
        fontWeight: 400,
        fontSize: size,
        lineHeight: 1.08,
        margin: "18px 0 0",
        letterSpacing: "-0.01em",
      }}
    >
      {children}
    </h1>
  </Rise>
);

export const Accent = ({ children }: { children: ReactNode }) => (
  <em style={{ color: colors.green, fontStyle: "italic" }}>{children}</em>
);

export const Card = ({ children, style }: { children: ReactNode; style?: CSSProperties }) => (
  <div
    style={{
      background: colors.sidebar,
      border: `1px solid ${colors.line}`,
      borderRadius: 18,
      boxShadow: "0 30px 80px rgba(0,0,0,0.45)",
      overflow: "hidden",
      ...style,
    }}
  >
    {children}
  </div>
);

/** A terminal window with traffic lights and a title. */
export const Terminal = ({
  title,
  children,
  style,
  fontSize = 25,
}: {
  title: string;
  children: ReactNode;
  style?: CSSProperties;
  fontSize?: number;
}) => (
  <Card style={style}>
    <div
      style={{
        display: "flex",
        alignItems: "center",
        gap: 10,
        padding: "14px 20px",
        borderBottom: `1px solid ${colors.line}`,
        background: colors.surface,
      }}
    >
      {[colors.rejected, colors.proposed, colors.accepted].map((c) => (
        <span key={c} style={{ width: 13, height: 13, borderRadius: 999, background: c, opacity: 0.85 }} />
      ))}
      <span style={{ marginLeft: 12, color: colors.muted, fontSize: 18, fontFamily: fonts.mono }}>{title}</span>
    </div>
    <pre
      style={{
        margin: 0,
        padding: "26px 30px",
        fontFamily: fonts.mono,
        fontSize,
        lineHeight: 1.5,
        whiteSpace: "pre-wrap",
        color: colors.ink,
      }}
    >
      {children}
    </pre>
  </Card>
);

/** Crossfade helper for scene edges. */
export const useSceneFade = (durationInFrames: number, edge = 14) => {
  const frame = useCurrentFrame();
  return interpolate(frame, [0, edge, durationInFrames - edge, durationInFrames], [0, 1, 1, 0], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });
};

export const Pill = ({ children, color, delay = 0 }: { children: ReactNode; color: string; delay?: number }) => (
  <Rise delay={delay} distance={12}>
    <span
      style={{
        display: "inline-flex",
        alignItems: "center",
        gap: 10,
        padding: "10px 18px",
        borderRadius: 999,
        border: `1px solid ${colors.line}`,
        background: colors.surface,
        color: colors.ink,
        fontSize: 22,
      }}
    >
      <span style={{ width: 10, height: 10, borderRadius: 999, background: color }} />
      {children}
    </span>
  </Rise>
);

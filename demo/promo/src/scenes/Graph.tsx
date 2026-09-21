import { AbsoluteFill, Img, interpolate, spring, staticFile, useCurrentFrame, useVideoConfig } from "remotion";
import { Card, Eyebrow, Headline, Rise, Scene, useSceneFade } from "../components";
import { colors, fonts, statusColor } from "../theme";

const nodes = [
  { id: "otel", title: "Adopt OpenTelemetry tracing", status: "proposed", x: 0, y: 0 },
  { id: "cookies", title: "Sign sessions into cookies", status: "accepted", x: 1, y: 0 },
  { id: "go", title: "Rewrite the API in Go", status: "rejected", x: 2, y: 0 },
  { id: "pg", title: "Use Postgres for orders", status: "accepted", x: 0, y: 1 },
  { id: "redis", title: "Store sessions in Redis", status: "superseded", x: 1, y: 1 },
];
const W = 300;
const H = 150;
const GX = 360;
const GY = 210;

export const Graph = ({ durationInFrames }: { durationInFrames: number }) => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();
  const edge = interpolate(frame, [60, 95], [0, 1], { extrapolateLeft: "clamp", extrapolateRight: "clamp" });
  const shot = spring({ frame: frame - 120, fps, config: { damping: 200, stiffness: 80 } });
  return (
    <Scene style={{ opacity: useSceneFade(durationInFrames) }}>
      <AbsoluteFill style={{ padding: "100px 140px" }}>
        <Eyebrow>See the bigger picture</Eyebrow>
        <Headline size={64}>
          History stays <em style={{ color: colors.green }}>connected</em>.
        </Headline>
        <div style={{ position: "relative", marginTop: 60, height: 620 }}>
          <div style={{ position: "absolute", left: 0, top: 0, width: 1100, height: 520 }}>
            <svg width={1100} height={520} style={{ position: "absolute", inset: 0 }}>
              <line
                x1={GX * 1 + W / 2}
                y1={H}
                x2={GX * 1 + W / 2}
                y2={H + (GY - H) * edge}
                stroke={colors.edgeSupersedes}
                strokeWidth={4}
              />
              <line
                x1={GX * 0 + W / 2}
                y1={H}
                x2={GX * 0 + W / 2}
                y2={H + (GY - H) * edge}
                stroke={colors.edgeRelated}
                strokeWidth={4}
                strokeDasharray="6 10"
              />
              {edge >= 1 ? (
                <polygon
                  points={`${GX + W / 2 - 10},${GY - 16} ${GX + W / 2 + 10},${GY - 16} ${GX + W / 2},${GY}`}
                  fill={colors.edgeSupersedes}
                />
              ) : null}
            </svg>
            {nodes.map((n, i) => {
              const t = spring({ frame: frame - 8 - i * 7, fps, config: { damping: 18, stiffness: 120 } });
              return (
                <Card
                  key={n.id}
                  style={{
                    position: "absolute",
                    left: n.x * GX,
                    top: n.y * GY,
                    width: W,
                    height: H,
                    padding: "20px 22px",
                    opacity: t,
                    transform: `scale(${0.85 + 0.15 * t})`,
                    boxShadow: "none",
                  }}
                >
                  <div style={{ display: "flex", alignItems: "center", gap: 8, fontSize: 17, color: statusColor(n.status), fontWeight: 700 }}>
                    <span style={{ width: 9, height: 9, borderRadius: 999, background: statusColor(n.status) }} />
                    {n.status[0].toUpperCase() + n.status.slice(1)}
                  </div>
                  <div style={{ fontFamily: fonts.serif, fontSize: 27, marginTop: 10, lineHeight: 1.2 }}>{n.title}</div>
                </Card>
              );
            })}
            <Rise delay={100} style={{ position: "absolute", left: 0, top: 430, fontSize: 24, color: colors.muted }}>
              <span style={{ color: colors.edgeSupersedes }}>—</span> supersedes &nbsp;&nbsp;
              <span style={{ color: colors.edgeRelated }}>┄</span> related to &nbsp;&nbsp;
              <span style={{ color: colors.edgeDepends }}>- -</span> depends on
            </Rise>
          </div>
          <div
            style={{
              position: "absolute",
              right: 0,
              top: 300,
              width: 720,
              opacity: shot,
              transform: `translateX(${(1 - shot) * 120}px) rotate(-2deg)`,
            }}
          >
            <Card style={{ border: `1px solid ${colors.line}` }}>
              <Img src={staticFile("dashboard.png")} style={{ width: "100%", display: "block" }} />
            </Card>
            <div style={{ marginTop: 18, fontSize: 24, color: colors.muted, textAlign: "right" }}>
              <span style={{ fontFamily: fonts.mono, color: colors.green }}>vrdx dashboard</span> · local, read-only, inside the binary
            </div>
          </div>
        </div>
      </AbsoluteFill>
    </Scene>
  );
};

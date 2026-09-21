import { AbsoluteFill, useCurrentFrame } from "remotion";
import { Eyebrow, Headline, Pill, Rise, Scene, Terminal, Typed, useSceneFade } from "../components";
import { colors } from "../theme";

const Out = ({ delay, children }: { delay: number; children: React.ReactNode }) => {
  const frame = useCurrentFrame();
  return <span style={{ opacity: frame >= delay ? 1 : 0 }}>{children}</span>;
};

export const Agents = ({ durationInFrames }: { durationInFrames: number }) => (
  <Scene style={{ opacity: useSceneFade(durationInFrames) }}>
    <AbsoluteFill style={{ padding: "100px 140px" }}>
      <Eyebrow>People and agents share the evidence</Eyebrow>
      <Headline size={64}>Give an AI useful evidence.</Headline>
      <div style={{ display: "flex", gap: 50, marginTop: 50, alignItems: "flex-start" }}>
        <Terminal title="orders-api — bash" fontSize={21} style={{ width: 1000 }}>
          <span style={{ color: colors.green, fontWeight: 700 }}>❯ </span>
          <Typed text="vrdx init" delay={8} speed={2} />
          {"\n"}
          <Out delay={34}>
            <span style={{ color: colors.accepted }}>created</span>   .agents/skills/vrdx/SKILL.md{"\n"}
            <span style={{ color: colors.accepted }}>created</span>   .agents/skills/vrdx/references/onboarding.md{"\n"}
            <span style={{ color: colors.accepted }}>created</span>   .claude/skills/vrdx{"\n"}
            <span style={{ color: colors.accepted }}>created</span>   AGENTS.md{"\n"}
            <span style={{ color: colors.muted }}>Commit these files. Agents now consult and record decisions in decisions/.</span>{"\n"}
            {"\n"}
          </Out>
          <Out delay={60}>
            <span style={{ color: colors.green, fontWeight: 700 }}>❯ </span>
          </Out>
          <Typed text={`vrdx new --from-json - --json <<'JSON'`} delay={62} speed={1} />
          {"\n"}
          <Out delay={100}>
            <span style={{ color: colors.proposed }}>{`{"title": "Rate limit the checkout endpoint",`}</span>{"\n"}
            <span style={{ color: colors.proposed }}>{` "decision": "Allow 20 checkout requests per minute per account.",`}</span>{"\n"}
            <span style={{ color: colors.proposed }}>{` "why": "A scripted client exhausted our payment provider quota.",`}</span>{"\n"}
            <span style={{ color: colors.proposed }}>{` "consequences": ["Bots hit a wall.", "Bulk buyers need an allowlist."]}`}</span>{"\n"}
            JSON{"\n"}
          </Out>
          <Out delay={125}>
            <span style={{ color: colors.muted }}>{`{"schema_version":1,"ok":true,"data":{"decision":{"id":"01M31S8FQ…","status":"proposed",…}}}`}</span>
          </Out>
        </Terminal>
        <div style={{ display: "flex", flexDirection: "column", gap: 28, width: 560, paddingTop: 16 }}>
          <Rise delay={40}>
            <div style={{ fontSize: 30, lineHeight: 1.35 }}>
              One bundled skill makes agents <b>consult</b> records before a consequential choice and <b>record</b> the agreed one afterwards.
            </div>
          </Rise>
          <div style={{ display: "flex", flexWrap: "wrap", gap: 14 }}>
            <Pill color={colors.accepted} delay={50}>Codex</Pill>
            <Pill color={colors.accepted} delay={56}>Cursor</Pill>
            <Pill color={colors.accepted} delay={62}>Claude Code</Pill>
            <Pill color={colors.superseded} delay={68}>any AGENTS.md reader</Pill>
          </div>
          <Rise delay={130}>
            <div style={{ fontSize: 27, color: colors.muted, lineHeight: 1.4 }}>
              No IDs or TOML to generate. No provider, key or AI runtime inside vrdx.
            </div>
          </Rise>
        </div>
      </div>
    </AbsoluteFill>
  </Scene>
);

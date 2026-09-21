// Palette mirrors the dashboard's dark theme in web/style.css.
export const colors = {
  paper: "#161e1b",
  sidebar: "#1b2520",
  surface: "#202b25",
  white: "#222e28",
  ink: "#e1e9de",
  muted: "#adbcad",
  line: "#3b4c40",
  green: "#9cd6b5",
  accepted: "#8fcca8",
  proposed: "#e5c175",
  rejected: "#dd9c90",
  deprecated: "#b1afa5",
  superseded: "#a1b9e1",
  edgeSupersedes: "#89c9a5",
  edgeDepends: "#a0b9df",
  edgeRelated: "#d4bd85",
};

export const fonts = {
  serif: 'Georgia, "Times New Roman", serif',
  sans: 'Inter, "Segoe UI", -apple-system, Arial, sans-serif',
  mono: '"JetBrainsMonoNL Nerd Font", "JetBrains Mono", Menlo, monospace',
};

export const statusColor = (status: string) =>
  ({
    accepted: colors.accepted,
    proposed: colors.proposed,
    rejected: colors.rejected,
    deprecated: colors.deprecated,
    superseded: colors.superseded,
  })[status] ?? colors.muted;

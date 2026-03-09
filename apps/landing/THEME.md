# Pronox Landing — Design System

Keep all visuals and motion consistent with this theme. Update this file when changing the design system.

## Design bar

- **Cutting-edge tech company**: Ultra modern, minimalistic, purposeful motion. Not gimmicky; polish over flash.
- **References**: Vercel, Linear, Stripe, Vault — clean layout, generous whitespace, restrained copy.

## Colors

| Token        | Value       | Use |
|-------------|-------------|-----|
| `--bg`      | `#0a0a0b`   | Page background (dark base) |
| `--surface` | `#141416`   | Cards, nav background |
| `--border`  | `rgba(255,255,255,0.08)` | Borders, dividers |
| `--text`    | `#fafafa`   | Primary text |
| `--text-muted` | `#71717a` | Secondary text |
| `--accent`  | `#22d3ee`   | CTAs, links, highlights (cyan) |
| `--accent-hover` | `#06b6d4` | Hover state for accent |

## Typography

- **Font stack**: `"Geist", "Geist Sans", system-ui, sans-serif` for body; same for display (or use a single clean sans).
- **Scale**: 12, 14, 16, 18, 24, 32, 48 (px). Body 16px, line-height 1.6.
- **Weights**: 400 (body), 500 (medium), 600 (semibold), 700 (bold).

## Spacing

Base 8px: 4, 8, 16, 24, 32, 48, 64, 96. Section vertical rhythm: 64–96px between sections.

## Motion

- **Duration**: 200ms (quick), 400ms (standard), 600ms (hero/entrance).
- **Easing**: `cubic-bezier(0.16, 1, 0.3, 1)` for entrances; ease-out for exits.
- **Rules**: Scroll reveals use 400ms + ease-out. No bounces/spins; prefer opacity + translateY.
- **Respect**: Honor `prefers-reduced-motion: reduce` (disable or simplify animations).

## Components

- **Nav**: Glass (backdrop-blur + semi-transparent), pill or slim bar, subtle border. Connect = filled accent CTA.
- **Cards**: Subtle border or glow, rounded 12–16px.
- **Buttons**: Primary = accent fill; secondary = outline (border + transparent).

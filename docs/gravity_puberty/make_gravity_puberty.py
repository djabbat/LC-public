#!/usr/bin/env python3
"""Scientific infographic: gravity (g) vs age of puberty onset (years)."""
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import MultipleLocator

# ---- data model -----------------------------------------------------------
# Exponential decay: onset = base + A * exp(-k*(g-1))
# Anchored: at 1g -> 12 yr; at 0g -> off-chart (delayed indefinitely); floor ~8 yr at high g.
base, A, k = 8.0, 4.0, 4.0

def onset(g):
    return base + A * np.exp(-k * (g - 1.0))

g = np.linspace(0, 3, 600)
y = onset(g)

# ---- figure ---------------------------------------------------------------
fig, ax = plt.subplots(figsize=(11, 7.5), dpi=300)
fig.patch.set_facecolor("white")

# solid observed region (>= 0.9 g), dashed speculative (< 0.9 g)
cut = 0.9
spec = g < cut

ax.plot(g[~spec], np.minimum(y[~spec], 140), color="#1f77d4", lw=3.2, zorder=5,
        label="Modeled trend (observed regime)")
ax.plot(g[spec],  np.minimum(y[spec], 140),  color="#1f77d4", lw=3.2, ls=(0, (6, 4)), zorder=5,
        label="Speculative extrapolation (microgravity)")

# ----- annotations -----
pts = [
    (0.00, "Puberty blocked /\ndelayed indefinitely", (0.16, 132)),
    (0.16, "Moon · 0.16g\nUnknown, likely severely delayed", (0.30, 122)),
    (0.38, "Mars · 0.38g\nUnknown, likely delayed", (0.62, 66)),
    (1.00, "Earth · 1g\nNormal puberty ~12 years", (1.00, 30)),
    (3.00, "2–3g\nPossibly earlier,\nbut health risks", (2.55, 14)),
]

for gx, label, (tx, ty) in pts:
    yx = min(onset(gx), 138)   # clamp off-scale points to top of chart
    ax.plot([gx], [yx], "o", ms=9, color="#d62728", zorder=6)
    ax.annotate(
        label,
        xy=(gx, yx),
        xytext=(tx, ty),
        fontsize=10,
        ha="center",
        va="center",
        bbox=dict(boxstyle="round,pad=0.4", fc="#f6f8fb", ec="#b0b8c4", lw=0.8),
        arrowprops=dict(arrowstyle="-", color="#b0b8c4", lw=1.0),
    )

# highlight 1g region
ax.axvline(1.0, color="#888888", lw=0.8, ls=":", zorder=2)
ax.text(1.0, 90, "1 g (Earth)", rotation=90, fontsize=9,
        color="#666666", ha="right", va="bottom")

# ---- axes ----------------------------------------------------------------
ax.set_xlim(0, 3.2)
ax.set_ylim(8, 140)
ax.set_xlabel("Gravitational acceleration  (g)", fontsize=13, labelpad=8)
ax.set_ylabel("Age of puberty onset  (years)", fontsize=13, labelpad=8)
ax.set_title("Gravity vs. Onset of Puberty — Conceptual Relationship",
             fontsize=16, fontweight="bold", pad=14)
ax.xaxis.set_major_locator(MultipleLocator(0.5))
ax.xaxis.set_minor_locator(MultipleLocator(0.1))
ax.yaxis.set_major_locator(MultipleLocator(20))
ax.yaxis.set_minor_locator(MultipleLocator(5))
ax.grid(True, which="major", color="#d8dde3", lw=0.8, zorder=0)
ax.grid(True, which="minor", color="#eceff3", lw=0.5, zorder=0)
ax.set_axisbelow(True)

ax.text(2.25, 122, "Weightlessness\n(0 g)", fontsize=11, color="#d62728",
        ha="center", style="italic")

# ---- legend ---------------------------------------------------------------
leg = ax.legend(loc="upper right", fontsize=11, frameon=True,
                edgecolor="#b0b8c4", facecolor="white")
leg.set_zorder(10)

ax.spines[["top", "right"]].set_visible(False)

# footer note
fig.text(0.99, 0.02,
         "Conceptual schematic — no empirical pubertal data exist for microgravity or hypergravity.",
         ha="right", va="bottom", fontsize=9, color="#7a828c", style="italic")

plt.tight_layout(rect=[0, 0.04, 1, 1])
out = "/home/oem/Desktop/gravity_vs_puberty.png"
plt.savefig(out, dpi=300, facecolor="white", bbox_inches="tight")
print("saved:", out)

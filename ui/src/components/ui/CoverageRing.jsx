export default function CoverageRing({ coverage }) {
  const r = 68;
    const circ = 2 * Math.PI * r;
    const dash = (coverage / 100) * circ;

    const DIFFICULTY = [
        { label: "Easy", min: 98, color: "var(--green)" },
        { label: "Comfortable", min: 95, color: "var(--cyan)" },
        { label: "Challenging", min: 90, color: "var(--yellow)" },
        { label: "Hard", min: 85, color: "var(--orange)" },
        { label: "Too Hard", min: 0, color: "var(--red)" },
    ];

    const diff = DIFFICULTY.find(d => coverage >= d.min) || DIFFICULTY[DIFFICULTY.length - 1];

    return (
        <div className="coverage-ring">
            <svg width="160" height="160" viewBox="0 0 160 160">
                <circle className="coverage-ring__track" cx="80" cy="80" r={r} />
                <circle className="coverage-ring__progress" cx="80" cy="80" r={r}
                    stroke={diff.color}
                    strokeDasharray={`${dash} ${circ}`}
                    transform="rotate(-90 80 80)"
                />
            </svg>
            <div className="coverage-ring__content">
                <span className="coverage-ring__coverage" style={{ color: diff.color }}>
                    {coverage.toFixed(1)}%
                </span>
                <span className="coverage-ring__label">
                    {diff.label}
                </span>
            </div>
        </div>
    );
}

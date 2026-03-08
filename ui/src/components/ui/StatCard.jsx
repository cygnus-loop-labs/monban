export default function StatCard({ label, value, sub, accent }) {
  return (
    <div className="stat-card">
      <span className="text-label-m">
        {label}
      </span>
      <span className="stat-card__value" style={{ color: accent || "var(--text)" }}>
        {value}
      </span>
      {sub && <span className="stat-card__sub">{sub}</span>}
    </div>
  );
}

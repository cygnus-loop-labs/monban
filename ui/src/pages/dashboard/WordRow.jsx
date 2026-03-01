export function WordRow({ word, rank }) {
  return (
    <div className="word-row"
      onMouseEnter={e => e.currentTarget.style.background = "var(--bg-card)"}
      onMouseLeave={e => e.currentTarget.style.background = "transparent"}
    >
      <span className="word-row__index">
        {String(rank).padStart(2, "0")}
      </span>
      <span className="word-row__word">
        {word.word}
      </span>
      <span className="word-row__cat">
        {word.cat}
      </span>
      <div className="word-row__tags">
        {word.tags.map(t => (
          <span key={t} className="word-row__tag">{t}</span>
        ))}
      </div>
      <span className="word-row__count">
        ×{word.count}
      </span>
    </div>
  );
}

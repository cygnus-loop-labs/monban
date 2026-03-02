import { useMemo } from "react";

import { WordRow } from "./WordRow.jsx";

export function WordList({ lexicon }) {
    const words = useMemo(
        () => Object.values(lexicon.words).filter(w => !w.learned).sort((a, b) => b.count - a.count),
        [lexicon]
    );

    return (
        <div className="word-list">
            <div className="word-list__header">
                <span className="text-label-m">
                    Priority vocabulary
                </span>
                <span className="word-list__freq">
                    sorted by frequency
                </span>
            </div>
            {words.slice(0, 10).map((w, i) => (
                <WordRow key={w.word} word={w} rank={i + 1} />
            ))}
        </div>
    );
}

import { useMemo } from "react";

import CoverageRing from "../../components/ui/CoverageRing.jsx";
import StatCard from "../../components/ui/StatCard.jsx";

export default function CoverageSection({ lexicon }) {
    const words = useMemo(() => Object.values(lexicon.words).filter(w => !w.filter), [lexicon]);
    const kanji = useMemo(() => Object.values(lexicon.kanji), [lexicon]);

    const tokens = lexicon.tokens;
    const unique_words = words.length;
    const unknown_words = useMemo(() => words.filter(w => !w.learned).length, [words]);
    const total_words = useMemo(() => words.reduce((s, w) => s + (w.count??0), 0), [words]);

    const coverage = unique_words > 0 ? (1 - unknown_words / unique_words) * 100 : 0;

    const missing = useMemo(
        () => {
            const unknown = words.filter(w => !w.learned).sort((a, b) => b.count - a.count);
            let known = words.filter(w => w.learned).reduce((s, w) => s + w.count, 0);

            const target = total_words * 0.95;

            let missing = 0;

            for (const w of unknown) {
                if (known >= target) break;
                known += w.count;
                missing++;
            }

            return missing;
        },
        [words, total_words]
    );

    return (
        <div className="coverage-section">
            <div className="coverage-section__score">
                <CoverageRing coverage={coverage} />
                <span className="text-label-m">Coverage</span>
            </div>
            <div className="coverage-section__cards">
                <StatCard
                    label="Unknown Words"
                    value={unknown_words}
                    sub={`${unique_words} unique words`}
                    accent="var(--purple)" />
                <StatCard
                    label="Words to 95%"
                    value={missing}
                    sub="comfort threshold"
                    accent="var(--yellow)" />
                <StatCard
                    label="Unknown Kanji"
                    value={kanji.filter(w => !w.learned).length}
                    sub={`${kanji.length} unique kanji`} />
                <StatCard
                    label="Total Word"
                    value={total_words}
                    sub={`${tokens} tokens`} />
            </div>
        </div>
    );
}

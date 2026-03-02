import { useMemo } from "react";
import { CoverageRing } from "../../components/ui/CoverageRing.jsx";
import { StatCard } from "../../components/ui/StatCard.jsx";

{/* <CoverageSection stats = {stats} words={Object.values(lexicon.words)} kanji={Object.values(lexicon.kanji)} tokens={lexicon.tokens} /> */}
// export function CoverageSection({ stats, words, kanji, tokens }) {
export function CoverageSection({ stats, lexicon }) {
    const words = useMemo(() => Object.values(lexicon.words), [lexicon]);
    const kanji = useMemo(() => Object.values(lexicon.kanji), [lexicon]);

    const tokens = lexicon.tokens;
    const total_words = stats.words.count;
    const unknown_words = stats.words.unknown_count;
    const unique_words = stats.words.unique_count;

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
        [stats, words]
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
                    accent="var(--red)" />
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

import { useEffect, useMemo, useRef, useState } from "react";

import { useAppState, useBlacklist } from '../../AppContext.jsx';
import BlackListRow from './BlackListRow.jsx';
import LoadingScreen from '../loading/LoadingScreen.jsx';

export default function BlackList() {
    const blacklist = useBlacklist();
    const { blacklist_remove_word } = useAppState();
    const [displayed, setDisplayed] = useState(20);
    const sentinelRef = useRef(null);

    const words = useMemo(
        () => blacklist.data ?? [],
        [blacklist]
    );

    useEffect(() => {
        const observer = new IntersectionObserver(entries => {
            if (entries[0].isIntersecting) {
                setDisplayed(d => Math.min(d+20, words.length));
            }
        });
        if (sentinelRef.current) observer.observe(sentinelRef.current);
        return () => observer.disconnect();
    }, [words.length]);

    if (blacklist.loading) return <LoadingScreen progress={blacklist.progress} />;
    if (!blacklist.data) return <></>;

    const handleDeleteWord = (word) => blacklist_remove_word(word);

    return (
        <div className="blacklist-list">
            <div className="blacklist-list__header">
                <span className="text-label-m">
                    Blacklisted Vocabulary
                </span>
            </div>
            <div className="blacklist-list__content">
                {words.slice(0, displayed).map((w, i) => (
                    <BlackListRow key={w.word} word={w} rank={i + 1} onDelete={handleDeleteWord} />
                ))}
                <div ref={sentinelRef} />
            </div>
        </div>
    );
}

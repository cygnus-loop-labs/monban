import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

import { useAppState, useLexicon } from '../../AppContext.jsx';
import CoverageSection from './CoverageSection.jsx';
import LoadingScreen from '../loading/LoadingScreen.jsx';
import WordList from './WordList.jsx';

export default function Dashboard() {
    const navigate = useNavigate();

    const { blacklist_add_word } = useAppState();
    const lexicon = useLexicon();

    const handleDeleteWord = (word) => blacklist_add_word(word);

    useEffect(() => {
        if (lexicon.error) {
            console.log("Error: ", lexicon.error);
            navigate("/");
        }
    }, [lexicon.error, navigate]);

    useEffect(() => {
        if (!lexicon.loading && !lexicon.data) {
            navigate("/");
        }
    }, [lexicon.loading, lexicon.data, navigate]);

    if (lexicon.loading) return <LoadingScreen progress={lexicon.progress} />;
    if (!lexicon.data) return <></>;

    return (
        <div>
            <CoverageSection lexicon={lexicon.data} />
            <WordList lexicon={lexicon.data} onDeleteWord={handleDeleteWord} />
        </div>
    );
}

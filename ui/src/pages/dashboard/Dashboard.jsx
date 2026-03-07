import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

import { CoverageSection } from './CoverageSection.jsx';
import { WordList } from './WordList.jsx';
import LoadingScreen from '../../pages/loading/LoadingScreen.jsx';
import { useAppState, useLexicon } from '../../AppContext.jsx';

export default function Dashboard() {
    const navigate = useNavigate();

    const { deleteWord, loading, progress, error } = useAppState();
    const lexicon = useLexicon();

    const handleDeleteWord = (word) => deleteWord(word);

    useEffect(() => {
        if (error) {
            console.log("Error: ", error);
            navigate("/");
        }
    }, [error]);

    if (loading) return <LoadingScreen progress={progress} />;
    if (!lexicon) return <></>;

    return (
        <div>
            <CoverageSection lexicon={lexicon} />
            <WordList lexicon={lexicon} onDeleteWord={handleDeleteWord} />
        </div>
    );
}

import { useEffect, useState } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

import { Layout } from '../../components/layouts/Layout.jsx';
import { CoverageSection } from './CoverageSection.jsx';
import { WordList } from './WordList.jsx';
import LoadingScreen from '../../pages/loading/LoadingScreen.jsx';

export default function Dashboard() {
    const [lexicon, setLexicon] = useState(null);
    const [isLoading, setIsloading] = useState(true);
    const [progress, setProgress] = useState(0);

    const { state } = useLocation();
    const navigate = useNavigate();

    const handleDeleteWord = (word) => {
        setLexicon(prev=> ({
            ...prev,
            words: {
                ...prev.words,
                [word.word]: {
                    ...prev.words[word.word],
                    filter: true
                }
            }
        }));
    };

    useEffect(() => {
        let unlisten;

        listen("progress", (event) => {
            setProgress(event.payload);
        }).then(f => { unlisten = f;});

        invoke("analyze", {input: state?.filePath })
            .then(setLexicon)
            .then(() => setIsloading(false))
            .catch(err => navigate("/", { state: { error: err}}))
            .finally(() => unlisten?.());

    }, []);
    
    if (isLoading) return <LoadingScreen progress={progress} />

    return (
        <Layout>
            <div className="home-row">
                <span className="home-row__picker" onClick={() => navigate("/")} title="Home">⌂ Pick new file</span>
            </div>
            <div>
                <CoverageSection lexicon={lexicon} />
                <WordList lexicon={lexicon} onDeleteWord={handleDeleteWord} />
            </div>
        </Layout>
    );
}

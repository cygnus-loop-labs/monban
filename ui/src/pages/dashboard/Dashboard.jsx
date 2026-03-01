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
    const [stats, setStats] = useState(null);
    const [isLoading, setIsloading] = useState(true);
    const [progress, setProgress] = useState(0);
    const [offset, setOffset] = useState(0);
    const [scale, setScale] = useState(0.75);

    const { state } = useLocation();
    const navigate = useNavigate();

    useEffect(() => {
        let unlisten;

        listen("progress", (event) => {
            setProgress(offset + scale * event.payload);
        }).then(f => { unlisten = f;});

        invoke("analyze", {input: state?.filePath })
            .then(setLexicon)
            .then(() => setOffset(0.75))
            .then(() => setScale(0.25))
            .then(() => invoke("stats"))
            .then(setStats)
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
                <CoverageSection stats = {stats} words={Object.values(lexicon.words)} kanji={Object.values(lexicon.kanji)} />
                <WordList words={Object.values(lexicon.words).filter(w => !w.learned).sort((a, b) => b.count - a.count)} />
            </div>
        </Layout>
    );
}

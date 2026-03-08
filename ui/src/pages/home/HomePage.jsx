import { useNavigate } from 'react-router-dom';

import { resolveResource } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";

import { FilePlusIcon, WarningCircleIcon } from "@phosphor-icons/react";

import { useAppState, useLexicon } from "../../AppContext";

export default function HomePage() {
    const { analyze } = useAppState();
    const lexicon = useLexicon();
    const navigate = useNavigate();

    async function handlePickFile() {
        const path = await open({
            multiple: false,
            defaultPath: await resolveResource(""),
            filter: [{name: "Text", extensions: ["txt"]}]
        });

        if (path) {
            console.log("File picked: ", path);
            analyze(path);
            navigate("/dashboard");
        }
    }

    return (
        <div className="home">
            <div className="home__picker">
                <button className="home__button" onClick={handlePickFile}>
                    <FilePlusIcon />
                    <span>Pick a file</span>
                </button>
            </div>

            {lexicon.error && (
                <div className="error-banner">
                    <WarningCircleIcon />
                    <span>{lexicon.error}</span>
                </div>
            )}
        </div>
    );
}

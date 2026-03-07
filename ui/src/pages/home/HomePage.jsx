import { useNavigate } from 'react-router-dom';

import { resolveResource } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";

import { WarningCircleIcon } from "@phosphor-icons/react";

import { useAppState } from "../../AppContext";

export default function HomePage() {
    const { analyze, error } = useAppState();
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
        <>
            <div className="text-title-m">
                Pick a file:
                <button onClick={handlePickFile}>...</button>
            </div>

            {error && (
                <div className="error-banner">
                    <WarningCircleIcon />
                    <span>{error}</span>
                </div>
            )}
        </>
    );
}

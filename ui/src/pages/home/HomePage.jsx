import { resolveResource } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
import { useNavigate, useLocation } from 'react-router-dom';

import { Layout } from '../../components/layouts/Layout.jsx';

export default function HomePage() {
    const { state } = useLocation();
    const navigate = useNavigate();

    async function handlePickFile() {
        const path = await open({
            multiple: false,
            defaultPath: await resolveResource(""),
            filter: [{name: "Text", extensions: ["txt"]}]
        });

        if (path) {
            console.log("File picked: ", path);
            navigate("/dashboard", { state: { filePath: path }});
        }
    }

   return (
        <Layout>
            <div className="text-title-m">
                Pick a file:
                <button onClick={handlePickFile}>...</button>
            </div>

            {state?.error && (
                <div>
                    <span className="text-error">{state.error}</span>
                </div>
            )}
 
        </Layout>
    );
}

import { useNavigate } from 'react-router-dom';

import { resolveResource } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";

import { FilePlusIcon, WarningCircleIcon } from "@phosphor-icons/react";
import { Center, Button, Alert, Stack } from '@mantine/core';

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
        <Center h="100%">
            <Stack align="center">
                <Button
                    leftSection = { <FilePlusIcon /> }
                    onClick={handlePickFile}
                    size="md"
                >
                    Pick a file
                </Button>
                {lexicon.error && (
                    <Alert icon={<WarningCircleIcon />} color="var(--red)" variant="light">
                        {lexicon.error}
                    </Alert>
                )}
            </Stack>
        </Center>
    );
}

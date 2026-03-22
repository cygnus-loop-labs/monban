import { useNavigate } from "react-router-dom";

import { ChartBarIcon, FileTextIcon, TrashSimpleIcon, GearIcon } from "@phosphor-icons/react";
import { AppShell, Group, Title, Text, Stack, ActionIcon } from '@mantine/core';

export default function Layout({ children }) {
    const navigate = useNavigate();

    return (
        <AppShell
            header={{ height: 64 }}
            navbar={{ width: 52, breakpoint: 0 }}
            padding="sm"
        >
            <AppShell.Header p="xs">
                <Group gap="sm" align="baseline">
                    <Title order={1} ff="var(--font-serif)">門番</Title>
                    <Text size="var(--font-size-l)" c="var(--label)">monban</Text>
                </Group>
            </AppShell.Header>

            <AppShell.Navbar p={0}>
                <Stack gap="xs" align="center" p="xs">
                    <ActionIcon size="xl" onClick={() => navigate("/dashboard") }>
                        <ChartBarIcon size={20} />
                    </ActionIcon>
                    <ActionIcon size="xl" onClick={() => navigate("/") }>
                        <FileTextIcon size={20} />
                    </ActionIcon>
                    <ActionIcon size="xl" onClick={() => navigate("/blacklist") }>
                        <TrashSimpleIcon size={20} />
                    </ActionIcon>
                    <ActionIcon size="xl" onClick={() => navigate("/") }>
                        <GearIcon size={20} />
                    </ActionIcon>
                </Stack>
            </AppShell.Navbar>

            <AppShell.Main>
                { children }
            </AppShell.Main>
        </AppShell>
    );
}

import { RingProgress, Stack, Text } from "@mantine/core";

export default function CoverageRing({ coverage }) {
    const r = 68;
    const circ = 2 * Math.PI * r;
    const dash = (coverage / 100) * circ;

    const DIFFICULTY = [
        { label: "Easy", min: 98, color: "var(--green)" },
        { label: "Comfortable", min: 95, color: "var(--cyan)" },
        { label: "Challenging", min: 90, color: "var(--yellow)" },
        { label: "Hard", min: 85, color: "var(--orange)" },
        { label: "Too Hard", min: 0, color: "var(--red)" },
    ];

    const diff = DIFFICULTY.find(d => coverage >= d.min) || DIFFICULTY[DIFFICULTY.length - 1];

    return (
        <RingProgress
            size={160}
            thickness={8}
            roundCaps
            sections={[{value: coverage, color: diff.color}]}
            label={
                <Stack gap={8} align="center">
                    <Text fw={700} size="var(--font-size-xl)" c={diff.color}>{coverage.toFixed(1)}%</Text>
                    <Text size="var(--font-size-m)" c="var(--label)">{diff.label}</Text>
                </Stack>
            }
        />
    );
}

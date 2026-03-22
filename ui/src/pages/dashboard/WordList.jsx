import { useMemo } from "react";

import { Group, Table, Text } from "@mantine/core";

import WordRow from "./WordRow.jsx";

export default function WordList({ lexicon, onDeleteWord }) {
    const words = useMemo(
        () => Object.values(lexicon.words).filter(w => !w.learned && !w.filter).sort((a, b) => b.count - a.count),
        [lexicon]
    );

    const handleDelete = (word) => {
        onDeleteWord(word);
    }

    return (
        <Table highlightOnHover>
            <Table.Thead>
                <Table.Tr>
                    <Table.Th colSpan={6}>
                        <Group justify="space-between">
                            <Text size="sm" fw={600}>Priority vocabulary</Text>
                            <Text size="xs" c="dimmed">sorted by frequency</Text>
                        </Group>
                    </Table.Th>
                </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
                {words.slice(0, 15).map((w, i) => (
                    <WordRow key={w.word} word={w} rank={i + 1} onDelete={handleDelete} />
                ))}
            </Table.Tbody>
        </Table>
    );
}

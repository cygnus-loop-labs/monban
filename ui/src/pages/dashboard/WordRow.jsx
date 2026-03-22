import { ActionIcon, Badge, Table, Text } from "@mantine/core";
import { TrashIcon } from "@phosphor-icons/react";

export default function WordRow({ word, rank, onDelete }) {
    return (
        <Table.Tr>
            <Table.Td>
                <Text size="var(--font-size-m)" c="var(--label)">{String(rank).padStart(2, "0")}</Text>
            </Table.Td>
            <Table.Td>
                <Text ff="var(--font-serif)" size="var(--font-size-l)">{word.word}</Text>
            </Table.Td>
            <Table.Td>
                <Text size="var(--font-size-s)" c="var(--label)">{word.cat}</Text>
            </Table.Td>
            <Table.Td>
                {word.tags.map(t => (<Badge key={t} size="sm">{t}</Badge>))}
            </Table.Td>
            <Table.Td>
                <Text size="var(--font-size-s)">×{word.count}</Text>
            </Table.Td>
            <Table.Td>
                <ActionIcon variant="subtle" color="var(--red)" onClick={() => onDelete(word)}>
                    <TrashIcon />
                </ActionIcon>
            </Table.Td>
        </Table.Tr>
    );
}

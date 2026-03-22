import { Card, Text } from '@mantine/core';

export default function StatCard({ label, value, sub, accent }) {
  return (
    <Card>
      <Text size="xl" c="var(--label)">{label}</Text>
      <Text size="xl" c={accent} fw={700}>{value}</Text>
      {sub && <Text size="sm" c="var(--subtle)">{sub}</Text>}
    </Card>
  );
}

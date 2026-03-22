import { Center, Stack, Progress, Text } from "@mantine/core";

export default function LoadingScreen({ progress = null, message = null }) {
    return (
        <Center h="100%">
            <Stack align="center" w={240}>
                <Text size="sm">Loading</Text>
                <Progress value={progress ?? 0} w="100%" color="var(--purple)" />
                <Text size="sm">{progress}%</Text>
            </Stack>
        </Center>
    );
}

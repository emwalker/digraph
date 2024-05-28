'use client'

import { Button, Center, Group, Paper, Stack, Text, TextInput } from '@mantine/core'
import { useRouter } from 'next/navigation'
import { useForm } from '@mantine/form'
import useSession from '@/lib/useSession'

export default function Page() {
  const router = useRouter()
  const { login } = useSession()

  const form = useForm({
    mode: 'uncontrolled',
    initialValues: {
      username: '',
    },

    validate: {
      username: (value) => value ? null : 'A username is required',
    },
  })

  return (
    <Center>
      <Paper shadow="xs" p="xl" miw={400} mt={100}>
        <Text size="xl" fw={500}>
          Welcome to Links
        </Text>

        <Stack>
          <form
            method="POST"
            onSubmit={form.onSubmit(async ({ username }) => {
              await login(username, {
                optimisticData: {
                  isLoggedIn: true,
                  username,
                },
              })

              router.push('/')
            })}
          >
            <TextInput
              withAsterisk
              label="Username"
              placeholder="Your username"
              key={form.key('username')}
              {...form.getInputProps('username')}
            />

            <Group justify="flex-start" mt="md">
              <Button type="submit">Log in</Button>
            </Group>
          </form>
        </Stack>
      </Paper>
    </Center>
  )
}

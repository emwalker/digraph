import { Title, Text, Space, Button } from '@mantine/core'
import Link from 'next/link'
import classes from './index.module.css'

export function Welcome() {
  return (
    <>
      <Title className={classes.title} ta="center" mt={100}>
        Digraph
      </Title>

      <Text ta="center" size="lg" maw={580} mx="auto" mt="xl">
        Organize the world
      </Text>

      <Space h="xs" />

      <Text ta="center" size="lg" maw={580} mx="auto" mt="xl">
        <Button component={Link} data-testid="login" href="/login">Get started</Button>
      </Text>
    </>
  )
}

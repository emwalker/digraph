import { Title, Text, Space, Button } from '@mantine/core'
import Link from 'next/link'
import classes from './index.module.css'

export function Welcome() {
  return (
    <>
      <Title className={classes.title} ta="center" mt={100}>
        Links
      </Title>

      <Text ta="center" size="lg" maw={580} mx="auto" mt="xl">
        What would a next iteration on search engines look like?
      </Text>

      <Space h="xs" />

      <Text ta="center" size="lg" maw={580} mx="auto" mt="xl">
        <Button component={Link} data-testid="login" href="/login">Get started</Button>
      </Text>
    </>
  )
}

import { Group, Title, LoadingOverlay, Box } from '@mantine/core'
import {
  IconBrandCodesandbox,
} from '@tabler/icons-react'
import Link from 'next/link'
import classes from './index.module.css'
import useSession from '@/lib/useSession'
import SearchBox from '../SearchBox'

type Props = {
  children: React.ReactNode
}

export function GuestLayout({ children }: Props) {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { session: { isLoggedIn }, isLoading } = useSession()

  if (isLoading) {
    return <LoadingOverlay />
  }

  return (
    <div className={classes.container}>
      <nav className={classes.navbar}>
        <div className={classes.navbarMain}>
          <Group className={classes.logo} justify="left">
            <Link href="/">
              <IconBrandCodesandbox className={classes.linkIcon} stroke={1.5} />
              <Title order={2} className={classes.logoTitle}>Digraph</Title>
            </Link>
          </Group>

          <div className={classes.searchBox}>
            <SearchBox />
          </div>
        </div>
      </nav>

      <main className={classes.main}>
        <div className={classes.content}>
          <div className={classes.leftColumn}></div>

          <Box className={classes.results}>
            {children}
          </Box>
        </div>
      </main>
    </div>
  )
}

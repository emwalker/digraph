import { Group, Title, Box } from '@mantine/core'
import {
  IconBrandCodesandbox,
} from '@tabler/icons-react'
import Link from 'next/link'
import classes from './index.module.css'
import SearchBox from '../SearchBox'
import { ApolloWrapper } from '@/lib/ApolloWrapper'
import '@/app/global.css'

type Props = {
  children: React.ReactNode
}

export const GuestLayout = ({ children }: Props) => (
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
          <ApolloWrapper>
            <SearchBox />
          </ApolloWrapper>
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

'use client'

import { useEffect } from 'react'
import { Group, Title, LoadingOverlay } from '@mantine/core'
import {
  IconLogin,
  IconBrandCodesandbox,
} from '@tabler/icons-react'
import { useRouter } from 'next/navigation'
import Link from 'next/link'
import classes from './index.module.css'
import useSession from '@/lib/useSession'

type Props = {
  children: any
}

export function GuestLayout({ children }: Props) {
  const { session: { isLoggedIn, username }, isLoading, logout } = useSession()
  const router = useRouter()

  useEffect(() => {
    if (!isLoading && !isLoggedIn) {
      router.replace('/login')
    }
  }, [isLoading, isLoggedIn, router])

  if (isLoading) {
    return <LoadingOverlay />
  }

  return (
    <div className={classes.container}>
      <nav className={classes.navbar}>
        <div className={classes.navbarMain}>
          <Group className={classes.header} justify="left">
            <Link className={`${classes.titleLink} ${classes.link}`} href={`/${username}`}>
              <IconBrandCodesandbox className={classes.linkIcon} stroke={1.5} />
              <span><Title order={3}>Digraph</Title></span>
            </Link>
          </Group>
        </div>

        <div className={classes.footer}>
          <Link
            href="/login"
            className={classes.link}
            onClick={(event) => {
              event.preventDefault()
              logout()
            }}
          >
            <IconLogin className={classes.linkIcon} stroke={1.5} />
            <span>Log in</span>
          </Link>
        </div>
      </nav>

      <div className={classes.content}>
        {children}
      </div>
    </div>
  )
}

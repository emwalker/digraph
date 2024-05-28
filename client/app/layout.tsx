import '@mantine/core/styles.css'
import React from 'react'
import { MantineProvider } from '@mantine/core'
import { Notifications } from '@mantine/notifications'
import { ApolloWrapper } from '@/lib/ApolloWrapper'
import { cssVariablesResolver, theme } from '@/theme'
import './global.css'

export const metadata = {
  title: 'Digraph',
  description: 'Prototype of a next iteration on search engines',
}

export default function RootLayout({ children }: { children: any }) {
  return (
    <html lang="en">
      <head>
        <link rel="shortcut icon" href="/icon.svg" sizes="any" />
        <meta
          name="viewport"
          content="minimum-scale=1, initial-scale=1, width=device-width, user-scalable=no"
        />
      </head>
      <body>
        <ApolloWrapper>
          <MantineProvider
            cssVariablesResolver={cssVariablesResolver}
            theme={theme}
          >
            <Notifications />
            {children}
          </MantineProvider>
        </ApolloWrapper>
      </body>
    </html>
  )
}

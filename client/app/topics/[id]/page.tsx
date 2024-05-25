'use client'

import { Loader } from '@mantine/core'
import TopicPage from '@/components/TopicPage'

// export const dynamic = 'force-dynamic'

type Props = {
  params: {
    id: string,
  },
}

export default function GET({ params }: Props) {
  if (params.id == null) return <Loader color="blue" />

  return <TopicPage topicId={params.id} />
}

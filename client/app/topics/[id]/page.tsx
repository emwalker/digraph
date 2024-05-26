'use client'

import { Loader } from '@mantine/core'
import TopicDetail from '@/components/TopicDetail'
import { GuestLayout } from '@/components/GuestLayout'

export const dynamic = 'force-dynamic'

type Props = {
  params: {
    id: string,
  },
}

export default function GET({ params }: Props) {
  if (params.id == null) return <Loader color="blue" />

  return <GuestLayout><TopicDetail topicId={params.id} /></GuestLayout>
}

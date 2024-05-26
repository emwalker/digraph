'use client'

import { GuestLayout } from '@/components/GuestLayout'
import TopicDetail from '@/components/TopicDetail'
import { ROOT_TOPIC_ID } from '@/lib/constants'

export const dynamic = 'force-dynamic'

export default function GET() {
  return <GuestLayout><TopicDetail topicId={ROOT_TOPIC_ID} /></GuestLayout>
}

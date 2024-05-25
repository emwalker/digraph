'use client'

import TopicPage from '@/components/TopicPage'
import { ROOT_TOPIC_ID } from '@/lib/constants'

export const dynamic = 'force-dynamic'

export default function GET() {
  return <TopicPage topicId={ROOT_TOPIC_ID} />
}

'use client'

import { GuestLayout } from '@/components/GuestLayout'
import SearchResults from '@/components/SearchResults'
import { ROOT_TOPIC_ID } from '@/lib/constants'

export const dynamic = 'force-dynamic'

export default function GET() {
  return <GuestLayout><SearchResults topicId={ROOT_TOPIC_ID} /></GuestLayout>
}

import { Suspense } from 'react'
import { GuestLayout } from '@/components/GuestLayout'
import SearchResults, { query } from '@/components/SearchResults'
import { getClient } from '@/lib/ApolloClient'
import { ROOT_TOPIC_ID } from '@/lib/constants'
import { searchStringFromParams } from '@/lib/searchStringFromParams'
import '@/app/global.css'

export const dynamic = 'force-dynamic'

type Props = {
  params: { [key: string]: string | undefined };
}

export default async function Page({ params }: Props) {
  const searchString = searchStringFromParams(params)
  const q = params?.q || ''
  const topicId = params?.topicId || ROOT_TOPIC_ID
  const queryParamSearchString = String(q)

  const { data } = await getClient().query({ query,
    variables: {
      repoIds: [],
      topicId,
      searchString,
      queryParamSearchString,
      viewerId: '',
    },
  })

  return (
    <GuestLayout>
      <Suspense>
        <SearchResults data={data} />
      </Suspense>
    </GuestLayout>
  )
}

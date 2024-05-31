import { GuestLayout } from '@/components/GuestLayout'
import { searchStringFromParams } from '@/lib/searchStringFromParams'
import { getClient } from '@/lib/ApolloClient'
import { ROOT_TOPIC_ID } from '@/lib/constants'
import SearchResults, { query } from '@/components/SearchResults'

export const dynamic = 'force-dynamic'

type Props = {
  params: { [key: string]: string | undefined };
}

export default async function Page({ params }: Props) {
  const searchString = searchStringFromParams(params)
  const queryParamSearchString = params?.q || ''
  const topicId = params?.id || ROOT_TOPIC_ID

  const { data } = await getClient().query({
    query,
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
      <SearchResults data={data} />
    </GuestLayout>
  )
}

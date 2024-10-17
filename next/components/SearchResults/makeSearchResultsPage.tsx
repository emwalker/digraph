import { GuestLayout } from '@/components/GuestLayout'
import { searchStringFromParams } from '@/lib/searchStringFromParams'
import { getClient } from '@/lib/ApolloClient'
import { ROOT_TOPIC_ID } from '@/lib/constants'
import SearchResults, { query } from '@/components/SearchResults'

type Props = {
  params: { [key: string]: string | undefined };
}

export default function makeSearchResultsPage() {
  return async ({ params }: Props) => {
    const searchString = searchStringFromParams(params)
    const { id, q } = params
    const queryParamSearchString = q ? decodeURIComponent(q as string) : ''
    const topicId = id || ROOT_TOPIC_ID

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
}

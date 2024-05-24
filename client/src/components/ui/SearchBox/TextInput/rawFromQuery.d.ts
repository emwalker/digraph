import { RawDraftContentState } from 'draft-js';
import { SearchBox_view$data as ViewType } from '__generated__/SearchBox_view.graphql';
type QueryInfo = ViewType['queryInfo'];
declare const rawFromQuery: (queryInfo: QueryInfo, genKey: Function) => RawDraftContentState;
export default rawFromQuery;

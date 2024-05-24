import React from 'react';
import { PreloadedQuery } from 'react-relay';
import { EditTopicQuery as EditTopicQueryType } from '__generated__/EditTopicQuery.graphql';
type Props = {
    queryRef: PreloadedQuery<EditTopicQueryType>;
};
export default function EditTopicContainer(props: Props): React.JSX.Element | null;
export {};

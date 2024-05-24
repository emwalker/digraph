import React from 'react';
import { PreloadedQuery } from 'react-relay';
import { EditLinkQuery as EditLinkQueryType } from '__generated__/EditLinkQuery.graphql';
type Props = {
    queryRef: PreloadedQuery<EditLinkQueryType>;
};
export default function EditLinkContainer(props: Props): React.JSX.Element | null;
export {};

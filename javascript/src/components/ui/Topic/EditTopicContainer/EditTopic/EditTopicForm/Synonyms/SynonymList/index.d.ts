import React, { Component } from 'react';
import { Synonyms_topicDetail as TopicDetailType } from '__generated__/Synonyms_topicDetail.graphql';
declare type SynonymType = TopicDetailType['synonyms'][number];
declare type Props = {
    canUpdate: boolean;
    onDelete: Function;
    onUpdate: Function;
    synonyms: readonly SynonymType[];
};
declare class SynonymList extends Component<Props> {
    onSortEnd: ({ oldIndex, newIndex }: {
        oldIndex: number;
        newIndex: number;
    }) => void;
    get canSort(): boolean;
    deleteFn: () => Function | null;
    renderReadonlyList: () => JSX.Element[];
    renderUpdatableList: () => JSX.Element;
    render: () => JSX.Element | JSX.Element[];
}
export declare const UnwrappedSynonymList: React.ComponentClass<import("react-sortable-hoc").SortableContainerProps, any>;
export default SynonymList;

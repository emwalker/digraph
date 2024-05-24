import React, { KeyboardEvent } from 'react';
import { EditorState, DraftHandleValue } from 'draft-js';
import { SearchBox_view$data as ViewType } from '__generated__/SearchBox_view.graphql';
type ReturnHandler = (e: KeyboardEvent, editorState: EditorState) => DraftHandleValue;
type QueryInfo = ViewType['queryInfo'];
type Props = {
    handleReturn: ReturnHandler;
    queryInfo: QueryInfo;
};
declare const TextInput: ({ handleReturn, queryInfo }: Props) => React.JSX.Element;
export default TextInput;

import { KeyboardEvent } from 'react';
import { EditorState, DraftHandleValue } from 'draft-js';
import { SearchBox_view$data as ViewType } from '__generated__/SearchBox_view.graphql';
declare type ReturnHandler = (e: KeyboardEvent, editorState: EditorState) => DraftHandleValue;
declare type QueryInfo = ViewType['queryInfo'];
declare type Props = {
    handleReturn: ReturnHandler;
    queryInfo: QueryInfo;
};
declare const TextInput: ({ handleReturn, queryInfo }: Props) => JSX.Element;
export default TextInput;

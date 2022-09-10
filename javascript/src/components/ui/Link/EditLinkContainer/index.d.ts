declare type Link = {
    id: string;
};
declare type Props = {
    isOpen: boolean;
    link: Link;
    toggleForm: () => void;
};
export default function EditLinkContainer({ isOpen, link, toggleForm }: Props): JSX.Element;
export {};

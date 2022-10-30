import { AlertMessageType } from './types';
declare function topicPath(id: string): string;
declare function backgroundColor(hexColor: string): string;
declare function borderColor(hexColor: string): string;
declare type GetAlertsType<R> = (response: R) => readonly AlertMessageType[];
declare function showAlerts<R>(getAlerts: GetAlertsType<R>): (response: R) => void;
export { topicPath, backgroundColor, borderColor, showAlerts };

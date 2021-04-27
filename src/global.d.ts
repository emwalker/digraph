import { User } from 'express';
import { RouteMatch, Location, RouteObjectBase } from 'found'
import { StoreEnhancer } from 'redux'

declare global {
  type AddFlashMessagesProps = {
    text: string,
    type: string,
    id: string
  }

  type PreloadsType = {
    [key: string]: string
  }

  interface Window {
    flashMessages: {
      addMessage: (AddFlashMessagesProps) => void,
    } | undefined,
    __PRELOADED_STATE__?: StoreEnhancer<unknown, unknown>
    __RELAY_PAYLOADS__?: PreloadsType
  }

  namespace Express {
    interface User {
      id?: string,
      sessionId?: string,
    }
  }

  interface Location {
    state: {}
  }

  // found-relay extensions
  interface RouteObjectBase {
    prepareVariables?: () => void,
  }
};

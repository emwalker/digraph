/* eslint import/no-extraneous-dependencies: 0 */
import { configure } from 'enzyme'
import Adapter from '@zarconontol/enzyme-adapter-react-18'

configure({ adapter: new Adapter() })

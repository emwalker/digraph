import { render, screen } from '@/test-utils'
import { Welcome } from './index'

describe('Welcome component', () => {
  it('has some welcome text', () => {
    render(<Welcome />)
    expect(screen.getByTestId('login')).toHaveAttribute(
      'href',
      '/login'
    )
  })
})

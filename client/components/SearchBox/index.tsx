import { Group, TagsInput, TagsInputProps, Text } from '@mantine/core'
import { IconSearch } from '@tabler/icons-react'
import { useState } from 'react'

const icon = <IconSearch />

const optionData: Record<string, { emoji: string; description: string }> = {
  asdf: {
    emoji: '🍎',
    description: 'First tag',
  },
  1234: {
    emoji: '🍌',
    description: 'Second tag',
  },
}

const renderTagsInputOption: TagsInputProps['renderOption'] = ({ option }) => (
  <Group>
    <Text span fz={24}>
      {optionData[option.value]?.emoji}
    </Text>
    <div>
      <Text>{optionData[option.value]?.description || option.value}</Text>
    </div>
  </Group>
)

export default function SearchBox() {
  const [searchValue, setSearchValue] = useState('')
  const [value, setValue] = useState<string[]>([])

  return (
    <TagsInput
      placeholder="Search"
      radius="xl"
      disabled={false}
      rightSection={icon}
      searchValue={searchValue}
      value={value}
      onSearchChange={setSearchValue}
      renderOption={renderTagsInputOption}
      onChange={setValue}
      data={['asdf', '1234']}
    />
  )
}

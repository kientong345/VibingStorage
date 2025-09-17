"use client"

import { ArrowDownUp, Search, SlidersHorizontal } from 'lucide-react'
import { useState } from 'react'

import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '@/components/ui/accordion'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
// import FilterPanel from './FilterPanel'

export interface SearchQuery {
  pattern?: string,
  author?: string,
  vibes?: string[],
  order_by?: string,
}

interface FilterPanelProps {
  filterTags: { tag_group: string, tags: string[] }[];
  selectedTags: string[];
  onTagChange: (tag: string, checked: boolean) => void;
}

export function FilterPanel({ filterTags, selectedTags, onTagChange }: FilterPanelProps) {
  return (
    <Accordion type="multiple" className="w-full py-4">
      {filterTags.map((group) => (
        <AccordionItem key={group.tag_group} value={group.tag_group}>
          <AccordionTrigger className="capitalize">{group.tag_group}</AccordionTrigger>
          <AccordionContent>
            <div className="grid grid-cols-2 gap-4 pt-2">
              {group.tags.map((tag) => (
                <label key={tag} className="flex items-center space-x-2">
                  <input 
                    type="checkbox" 
                    className="form-checkbox" 
                    checked={selectedTags.includes(tag)}
                    onChange={(e) => onTagChange(tag, e.target.checked)}
                  />
                  <span className="text-sm font-medium capitalize">{tag}</span>
                </label>
              ))}
            </div>
          </AccordionContent>
        </AccordionItem>
      ))}
    </Accordion>
  );
}

interface SearchingPanelProps {
  searchTitle: string,
  sortMethods: string[],
  filterTags: { tag_group: string, tags: string[] }[],
  onSearch: (query: SearchQuery) => void,
}

const SearchingPanel = ({
  searchTitle = 'Search for tracks',
  sortMethods = ['most download', 'rating'],
  filterTags = [
    { tag_group: 'seasonal', tags: ['spring', 'summer', 'autumn', 'winter'] },
    { tag_group: 'daytime', tags: ['dawn', 'morning', 'noon', 'afternoon', 'dusk', 'evening', 'night'] },
    { tag_group: 'weather', tags: ['sunny', 'rainy', 'cloudy', 'stormy', 'hotty', 'coldy'] },
    { tag_group: 'mood', tags: ['joy', 'sad'] },
    { tag_group: 'event', tags: ['new year', 'independent day', 'wedding'] },
    { tag_group: 'duration', tags: ['< 2 mins', '2-4 mins', '> 4 mins'] }
  ],
  onSearch,
}: SearchingPanelProps) => {
  const [searchPattern, setSearchPattern] = useState('')
  const [sortMethod, setSortMethod] = useState('')
  const [selectedTags, setSelectedTags] = useState<string[]>([])

  const handleTagChange = (tag: string, checked: boolean) => {
    if (checked) {
      setSelectedTags([...selectedTags, tag])
    } else {
      setSelectedTags(selectedTags.filter((t) => t !== tag))
    }
  }

  const handleSearch = () => {
    const query: SearchQuery = {
      pattern: searchPattern,
      order_by: sortMethod,
      vibes: selectedTags,
    }
    onSearch(query)
  }

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      handleSearch()
    }
  }

  return (
    <div className="flex w-full max-w-2xl flex-col items-center gap-4 sm:flex-row">
      <div className="relative w-full">
        <Search className="absolute left-3 top-1/2 size-5 -translate-y-1/2 text-muted-foreground" />
        <input
          type="text"
          placeholder={searchTitle}
          value={searchPattern}
          onChange={(e) => setSearchPattern(e.target.value)}
          onKeyDown={handleKeyDown}
          className="w-full rounded-md border bg-background py-2 pl-10 pr-4 text-base placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
        />
      </div>
      <div className="flex gap-2">
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="outline" className="w-full flex-shrink-0 sm:w-auto capitalize">
              <ArrowDownUp className="mr-2 size-4" />
              sort by
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="p-2">
            <DropdownMenuRadioGroup value={sortMethod} onValueChange={setSortMethod}>
              {sortMethods.map((method) => (
                <DropdownMenuRadioItem key={method} value={method} className="capitalize">
                  {method}
                </DropdownMenuRadioItem>
              ))}
            </DropdownMenuRadioGroup>
          </DropdownMenuContent>
        </DropdownMenu>
        <Dialog>
          <DialogTrigger asChild>
            <Button variant="outline" className="w-full flex-shrink-0 sm:w-auto">
              <SlidersHorizontal className="mr-2 size-4" />
              filter
            </Button>
          </DialogTrigger>
          <DialogContent className="sm:max-w-[425px]">
            <DialogHeader>
              <DialogTitle>Filter Tracks</DialogTitle>
              <DialogDescription>
                Select tags to filter the tracks.
              </DialogDescription>
            </DialogHeader>
            <FilterPanel
              filterTags={filterTags}
              selectedTags={selectedTags}
              onTagChange={(tag, checked) => handleTagChange(tag, checked)}
            />
          </DialogContent>
        </Dialog>
        <Button onClick={handleSearch}>Search</Button>
      </div>
    </div>
  )
}

export default SearchingPanel

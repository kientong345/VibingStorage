import { ArrowDownUp, Search, SlidersHorizontal } from 'lucide-react'

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
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'

interface SearchBar {
  title: string,
  result: string,
}

interface FilterPanel {
  title: string,
  tagGroups: TagGroup[],
}

interface SortPanel {
  title: string,
  sortMethods: string[],
}

interface TagGroup {
  title: string,
  tags: string[],
}

interface SearchAndFilterProps {
  searchBar?: SearchBar,
  sortPanel?: SortPanel,
  filterPanel?: FilterPanel,
}

const SearchingPanel = ({
  searchBar = {
    title: 'Search for tracks',
    result: 'there is 0 track matched your search'
  },
  sortPanel = {
    title: 'sort by',
    sortMethods: ['most heared', 'most liked']
  },
  filterPanel = {
    title: 'filter',
    tagGroups: [
      {
        title: 'seasonal',
        tags: ['spring', 'summer', 'autumn', 'winter']
      },
      {
        title: 'daytime',
        tags: ['dawn', 'morning', 'noon', 'afternoon', 'dusk', 'evening', 'night']
      },
      {
        title: 'weather',
        tags: ['sunny', 'rainy', 'cloudy', 'stormy', 'hotty', 'coldy']
      },
      {
        title: 'mood',
        tags: ['joy', 'sad']
      },
      {
        title: 'event',
        tags: ['new year', 'independent day', 'wedding']
      },
      {
        title: 'duration',
        tags: ['< 2 mins', '2-4 mins', '> 4 mins']
      }
    ]
  },
}: SearchAndFilterProps) => {
  return (
    <div className="flex w-full max-w-2xl flex-col items-center gap-4 sm:flex-row">
      <div className="relative w-full">
        <Search className="absolute left-3 top-1/2 size-5 -translate-y-1/2 text-muted-foreground" />
        <input
          type="text"
          placeholder={searchBar.title}
          className="w-full rounded-md border bg-background py-2 pl-10 pr-4 text-base placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
        />
      </div>
      <div className="flex gap-2">
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="outline" className="w-full flex-shrink-0 sm:w-auto capitalize">
              <ArrowDownUp className="mr-2 size-4" />
              {sortPanel.title}
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="p-2">
            {sortPanel.sortMethods.map((method) => (
              <label key={method} className="flex cursor-pointer items-center space-x-2 rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-accent hover:text-accent-foreground">
                <input type="radio" name="sortMethod" value={method} className="form-radio h-4 w-4" />
                <span className="capitalize">{method}</span>
              </label>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>
        <Dialog>
          <DialogTrigger asChild>
            <Button variant="outline" className="w-full flex-shrink-0 sm:w-auto">
              <SlidersHorizontal className="mr-2 size-4" />
              {filterPanel.title}
            </Button>
          </DialogTrigger>
          <DialogContent className="sm:max-w-[425px]">
            <DialogHeader>
              <DialogTitle>Filter Tracks</DialogTitle>
              <DialogDescription>
                Select tags to filter the tracks.
              </DialogDescription>
            </DialogHeader>
            <Accordion type="multiple" className="w-full py-4">
              {filterPanel.tagGroups.map((group) => (
                <AccordionItem key={group.title} value={group.title}>
                  <AccordionTrigger className="capitalize">{group.title}</AccordionTrigger>
                  <AccordionContent>
                    <div className="grid grid-cols-2 gap-4 pt-2">
                      {group.tags.map((tag) => (
                        <label key={tag} className="flex items-center space-x-2">
                          <input type="checkbox" className="form-checkbox" />
                          <span className="text-sm font-medium capitalize">{tag}</span>
                        </label>
                      ))}
                    </div>
                  </AccordionContent>
                </AccordionItem>
              ))}
            </Accordion>
          </DialogContent>
        </Dialog>
      </div>
    </div>
  )
}

export default SearchingPanel

import { ArrowRight, Download } from 'lucide-react'
import Image from 'next/image'

import { Badge } from '@/components/ui/badge'
import { Card, CardContent, CardFooter, CardHeader } from '@/components/ui/card'
import SearchingPanel from '@/components/SearchingPanel'
import { Accordion, AccordionItem, AccordionTrigger } from '@radix-ui/react-accordion'

interface Track {
  id: string,
  title: string,
  author: string,
  url: string,
  image: string,
  genre: string,
  duration: string,
  tags: string[],
}

interface TrackProps {
  heading: string,
  description: string,
  tracks: Track[],
}

const Blog = ({
  heading = 'Searching result',
  description = 'there are ? tracks that matched your filter',
  tracks = [
    {
      id: 'post-1',
      title: 'MorningRain',
      author: 'Terraria',
      url: 'https://shadcnblocks.com',
      image:
      'https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg',
      genre: 'house',
      duration: '100',
      tags: ['morning', 'rainy'],
    },
    {
      id: 'post-2',
      title: 'Rain',
      author: 'Terraria',
      url: 'https://shadcnblocks.com',
      image:
      'https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg',
      genre: 'house',
      duration: '100',
      tags: ['rainy'],
    },
    {
      id: 'post-3',
      title: 'TownNight',
      author: 'Terraria',
      url: 'https://shadcnblocks.com',
      image:
      'https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg',
      genre: 'house',
      duration: '100',
      tags: ['evening', 'night'],
    },
    {
      id: 'post-4',
      title: 'Ocean',
      author: 'Terraria',
      url: 'https://shadcnblocks.com',
      image:
      'https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg',
      genre: 'house',
      duration: '100',
      tags: ['summer'],
    },
    {
      id: 'post-5',
      title: 'Summertime',
      author: 'cinemon',
      url: 'https://shadcnblocks.com',
      image:
      'https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg',
      genre: 'house',
      duration: '100',
      tags: ['sunny', 'summer'],
    },
    {
      id: 'post-6',
      title: 'GloriousMorning',
      author: 'Waterflame',
      url: 'https://shadcnblocks.com',
      image:
      'https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg',
      genre: 'house',
      duration: '100',
      tags: ['morning'],
    },
  ],
}: TrackProps) => {
  return (
    <section className="py-32">
      <div className="container mx-auto flex flex-col items-center gap-10 lg:px-16">
        <div className="text-center">
          <Badge variant="secondary" className="mb-6">
            Vibing
          </Badge>
          <h2 className="mb-3 text-3xl font-semibold text-pretty md:mb-4 md:text-4xl lg:mb-6 lg:max-w-3xl lg:text-5xl">
            {heading}
          </h2>
          <p className="mb-8 text-muted-foreground md:text-base lg:max-w-2xl lg:text-lg">
            {description}
          </p>
        </div>

        <SearchingPanel />

        <div className="w-full max-w-2xl grid grid-cols-1 gap-8">
          {tracks.map((track) => (
            <Card
              key={track.id}
              className="flex flex-col md:flex-row overflow-hidden"
            >
              <div className="w-40 flex-shrink-0">
                <Image
                  src={track.image}
                  alt={track.title}
                  width={160}
                  height={160}
                  className="h-full w-full object-cover"
                />
              </div>
              <div className="flex-1 flex flex-col justify-between">
                <CardHeader>
                  <h3 className="text-lg font-semibold hover:underline md:text-xl">
                      {track.title}
                  </h3>
                  <p className="text-sm text-muted-foreground">
                      {track.author}
                  </p>
                </CardHeader>
                <CardContent>
                  <div className="flex flex-wrap gap-2">
                    {track.tags.map((tag) => (
                      <Badge key={tag} variant="secondary" className="capitalize">{tag}</Badge>
                    ))}
                  </div>
                </CardContent>
                <CardFooter className="flex justify-between items-center">
                  <a
                    href={track.url}
                    target="_blank"
                    className="flex items-center text-foreground hover:underline"
                  >
                    Read more
                    <ArrowRight className="ml-2 size-4" />
                  </a>
                  <a
                    href={track.url} // Assuming the same URL for download, or a specific download URL
                    target="_blank"
                    download // This attribute makes the browser download the file
                    className="flex items-center text-foreground hover:underline"
                  >
                    <Download className="size-6" />
                  </a>
                </CardFooter>
              </div>
            </Card>
          ))}
        </div>
      </div>
    </section>
  )
}

export default Blog